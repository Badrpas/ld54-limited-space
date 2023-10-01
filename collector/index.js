import fs from 'fs-extra';
import { watch } from 'chokidar';
import * as path from 'path';
import { fileURLToPath } from 'url';
import { Queue } from './queue.js';

const delay = ms => new Promise(r => setTimeout(r, ms));

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const q = new Queue;

const SRC_PATH = path.resolve(__dirname, '../src');
const dirs = [SRC_PATH + '/**/*.rs'];

const handle_file_change = async (file_path, _stats) => {
    await delay(100);
    q.push(async () => {
        try {
            await handle_file(file_path);
        } catch (e) {
            console.error(e);
        }
    });
    console.debug('Queued ', file_path);
};
watch(dirs, { ignored: /\/target\// })
    .on('change', handle_file_change)
    .on('add', handle_file_change);

console.log(`Watching files in`, dirs);

async function handle_file (file_path) {
    console.log('Handling ', file_path);
    const text = await fs.readFile(file_path, 'utf8');
    const plugins = get_plugins(text);
    const abs_module_path = get_module_path_absolute(file_path);

    const parent_info = await get_parent(file_path);
    if (!parent_info) {
        console.log(`Couldn't find parent info`);
        return;
    }

    await ensure_include(plugins, /::(\w+)$/.exec(abs_module_path)[1], parent_info);
}

function get_plugins (text) {
    return [...text
        .matchAll(/impl (bevy::prelude::)?Plugin for (\w+) {/g)]
        .map(([, , identifier]) => identifier);
}

function get_module_path_absolute (file_path) {
    const from_root = path.relative(SRC_PATH, file_path).replace(/\.rs/, '').replace(/\/mod$/, '');
    return 'crate::' + from_root.replaceAll('/', '::');
}

async function get_parent (file_path) {
    const is_mod_itself = /\/mod\.rs$/.test(file_path);
    let parent_mod_path = (
        is_mod_itself
            ? path.dirname(file_path.replace(/\/mod\.rs$/, ''))
            : path.dirname(file_path)
    ) + '/mod.rs';
    if (!await fs.exists(parent_mod_path)) {
        parent_mod_path = parent_mod_path.replace(/\/mod\.rs$/, '/lib.rs');
        if (!await fs.exists(parent_mod_path)) {
            console.log(`Couldn't find parent mod for ${file_path}`);
            return null;
        }
    }
    try {
        let text = await fs.readFile(parent_mod_path, 'utf8');
        return [parent_mod_path, text, get_plugins(text)];
    } catch (e) {
        console.error(e);
        return null;
    }
}

async function ensure_mod (module_name, parent_text) {
    if (module_name === 'lib') {
        console.log('It looks like a lib.rs to me - skipping');
        return [false];
    }
    if (!parent_text.includes(`mod ${module_name};`)) {
        const mod_line = `pub mod ${module_name};`;
        {
            let last;
            parent_text.replace(/(pub )?mod \w+;/g, (match, _pub, idx, _whole) => {
                last = [idx, match.length];
                return match;
            });
            if (last) {
                const idx = last[0] + last[1];
                const out = `${parent_text.slice(0, idx)}\n${mod_line}${parent_text.slice(idx)}`;
                console.log(`Added "${mod_line}"`);
                return [true, out];
            }
        }

        return [true, mod_line + '\n' + parent_text];
    } else {
        console.log(`Already present: "mod ${module_name};"`);
        return [false];
    }
}

async function ensure_include (plugins, module_name, [parent_path, text]) {
    let changed = false;

    const [mod_added, out_text] = await ensure_mod(module_name, text);
    if (mod_added) {
        changed = true;
        text = out_text;
    }

    for (const plugin of plugins) {

        const plugin_addition_str = `app.add_plugins(${module_name}::${plugin});`;
        if (!text.includes(plugin_addition_str)) {
            console.log(`Adding "${plugin_addition_str}"`);
            let added = false;
            text = text.replace(
                /(fn build\(&self, app: &mut (?:bevy::prelude::)?App\) {[\s\S]*?)(?:\n?\s*)?(})/gm,
                function (_match, start, end) {
                    added = true;
                    return start + '\n        ' + plugin_addition_str + '\n    ' + end;
                }
            );
            if (!added) {
                console.warn(`Didn't add the thing!`);
            } else {
                changed = true;
            }
        } else {
            console.log(`Already present: "${plugin_addition_str}"`);
        }
    }
    if (changed) {
        console.log(`Writing out to ${parent_path}`);
        await fs.writeFile(parent_path, text);
    }
}
