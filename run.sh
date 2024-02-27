#!/usr/bin/env bash

export JAVA_HOME=`/usr/libexec/java_home -v 1.8.0_242`
export DYLD_FALLBACK_LIBRARY_PATH=$JAVA_HOME/jre/lib/server/

java -cp ./target/rusted-javanese-1.0-SNAPSHOT.jar com.rusty.JavaJungleJuice
java -Djava.library.path=src/main/rust/lib/target/release -cp ./target/rusted-javanese-1.0-SNAPSHOT.jar com.rusty.JollyRogerRadio
java -Djava.library.path=src/main/rust/lib/target/release -cp ./target/rusted-javanese-1.0-SNAPSHOT.jar com.rusty.JuniorJetRiders

src/main/rust/binaries/target/release/RedRiverRebellion
src/main/rust/binaries/target/release/RealJumpingJamboree
src/main/rust/binaries/target/release/RegionalRailJet
