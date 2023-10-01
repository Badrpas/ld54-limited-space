mod kek {
    macro_rules! filter_cmd {
        ($cmds:ident, $pat:pat) => {
            $cmds.0.iter().filter(|(_, cmd)| match &cmd.op_code {
                $pat => true,
                _ => false,
            })
        };
    }
    pub(crate) use filter_cmd;

    macro_rules! get_or_skip {
        ($p:pat = $e:expr => $name:ident) => {
            if let $p = $e {
                $name
            } else {
                continue;
            }
        };
    }
    pub(crate) use get_or_skip;

    macro_rules! some_or_skip {
        ($e:expr) => {
            get_or_skip!(Some(x) = $e => x)
        }
    }
    pub(crate) use some_or_skip;

    macro_rules! ok_or_skip {
        ($e:expr) => {
            get_or_skip!(Ok(x) = $e => x)
        }
    }
    pub(crate) use ok_or_skip;

    macro_rules! get_or_ret {
        ($p:pat = $e:expr => $name:ident) => {
            get_or_ret!($p = $e => $name ; ())
        };
        ($p:pat = $e:expr => $name:ident ; $ret:expr) => {
            if let $p = $e {
                $name
            } else {
                return $ret;
            }
        };
    }
    pub(crate) use get_or_ret;

    macro_rules! some_or_ret {
        ($e:expr) => {
            some_or_ret!($e ; ())
        };
        ($e:expr ; $ret:expr) => {
            get_or_ret!(Some(x) = $e => x ; $ret)
        };
    }
    pub(crate) use some_or_ret;

    macro_rules! ok_or_ret {
        ($e:expr) => {
            ok_or_ret!($e ; ())
        };
        ($e:expr ; $ret:expr) => {
            get_or_ret!(Ok(x) = $e => x ; $ret)
        };
    }
    pub(crate) use ok_or_ret;
}

pub(crate) use kek::*;

