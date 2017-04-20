error_chain! {
    foreign_links {
        Connection(::diesel::result::ConnectionError);
        Database(::diesel::result::Error);
        EnvVar(::std::env::VarError);
        Io(::std::io::Error);
        PoolInit(::r2d2::InitializationError);
        PublicSuffix(::publicsuffix::errors::Error);
    }

    links {
        DotEnv(::dotenv::Error, ::dotenv::ErrorKind);
    }

    errors {
        DuplicateUser {
            description("Can't create a duplicate user")
            display("Can't create a duplicate user")
        }
        IncorrectPassword {
            description("Incorrect password")
            display("Incorrect password")
        }
        InvalidCardnum(n: String) {
            description("Invalid card number")
            display("Invalid card number: {:?}", n)
        }
        InvalidUsername(u: String) {
            description("Invalid username")
            display("Invalid username: {:?}", u)
        }
        Jwt(e: ::jwt::error::Error) {
            description("Couldn't create a token")
            display("{:?}", e)
        }
        NoSuchUser {
            description("No such user exists")
            display("No such user exists")
        }
    }
}
