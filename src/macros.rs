//! Utility macros

macro_rules! next {
    ($bytes:ident) => {{
        if $bytes.is_empty() {
            return Ok(Status::Partial);
        }
        $bytes.get_u8()
    }};
}

macro_rules! peek {
    ($bytes:ident) => {{
        match $bytes.get(0) {
            None => return Ok(Status::Partial),
            Some(b) => *b,
        }
    }};
}

macro_rules! expect {
    ($bytes:ident.next() == $pat:pat => $ret:expr) => {
        expect!(next!($bytes) => $pat |? $ret)
    };
    ($e:expr => $pat:pat |? $ret:expr) => {
        match $e {
            v@$pat => v,
            _ => return $ret
        }
    };
}

macro_rules! complete {
    ($e:expr) => {
        match $e? {
            Status::Complete(v) => v,
            Status::Partial => return Ok(Status::Partial),
        }
    };
}

macro_rules! byte_map {
    ($($flag:expr,)*) => ([
        $($flag != 0,)*
    ])
}

macro_rules! space {
    ($bytes:ident or $err:expr) => ({
        expect!($bytes.next() == b' ' => Err($err));
        $bytes.advance(1);
    })
}

macro_rules! newline {
    ($bytes:ident) => ({
        match next!($bytes) {
            b'\r' => {
                expect!($bytes.next() == b'\n' => Err(Error::NewLine));
            },
            b'\n' => { },
            _ => return Err(Error::NewLine)
        }
    })
}
