# rust_db_examples
database examples in rust

psql_demo works, but diesel_example does not.

* .env config 1
```
DATABASE_URL=postgres://tony:12345678@localhost:5432/tony?sslmode=disable
```

error message:
```
error: thread '<main>' panicked at 'Error connecting to postgres://tony:12345678@localhost:5432/tony?sslmode=disable: BadConnection("invalid connection option \"postgres://tony:12345678@localhost:5432/tony?sslmode\"\n")', ../src/libcore/result.rs:746
```

* .env config 2
```
DATABASE_URL=postgres://localhost/tony
```

error message:
```
thread '<main>' panicked at 'Error connecting to postgres://localhost/tony: BadConnection("missing \"=\" after \"postgres://localhost/tony\" in connection info string\n")', ../src/libcore/result.rs:746
```

# environment:
rust: 
```
rustc 1.9.0-nightly (241a9d0dd 2016-04-05)
```

os: ubuntu 12.04
```
Linux svr2 3.2.0-77-generic #114-Ubuntu SMP Tue Mar 10 17:26:03 UTC 2015 x86_64 x86_64 x86_64 GNU/Linux
```

gcc:
```
gcc version 4.6.3 (Ubuntu/Linaro 4.6.3-1ubuntu5) 
```

postgresql: 
```
PostgreSQL 9.1.18 on x86_64-unknown-linux-gnu, compiled by gcc (Ubuntu/Linaro 4.6.3-1ubuntu5) 4.6.3, 64-bit
```

libpq:
```
Package: libpq-dev
Priority: optional
Section: libdevel
Installed-Size: 880
Maintainer: Ubuntu Developers <ubuntu-devel-discuss@lists.ubuntu.com>
Original-Maintainer: Martin Pitt <mpitt@debian.org>
Architecture: amd64
Source: postgresql-9.1
Version: 9.1.21-0ubuntu0.12.04
Replaces: postgresql-dev
Depends: libc6 (>= 2.4), libpq5 (= 9.1.21-0ubuntu0.12.04), libssl-dev, libkrb5-dev, comerr-dev
```
