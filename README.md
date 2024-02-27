## Tiny JNi benchmark in Java/Rust

# Build

```shell
export JAVA_HOME=`/usr/libexec/java_home -v 1.8.0_242`
mvn package
cd src/main/rust/binaries;cargo clean; cargo  build --release; cd -
cd src/main/rust/lib/;cargo clean; cargo  build --release; cd -
```

# Run
```shell
$ ./run.sh
Java -> Java -> Java : res = 5000000 Total execution time for JNI transitions (    ) = 0                : 9604ms
Java -> Rust -> Rust : res = 5000000 Total execution time for JNI transitions (J->R) = 100000           : 6481ms
Java -> Java -> Rust : res = 5000000 Total execution time for JNI transitions (J->R) = 10000000000      : 86966ms
Rust -> Rust -> Rust : res = 5000000 Total execution time for JNI transitions (    ) = 0                : 6125ms
Rust -> Java -> Java : res = 5000000 Total execution time for JNI transitions (R->J) = 100000           : 9985ms
Rust -> Rust -> Java : res = 5000000 Total execution time for JNI transitions (R->J) = 10000000000      : 1424496ms
```
