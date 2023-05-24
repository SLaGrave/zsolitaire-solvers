cmake_minimum_required(VERSION 3.10)

project(Cribbage)

execute_process(COMMAND cargo build WORKING_DIRECTORY ../cribbage-rs)
execute_process(COMMAND cbindgen --config cbindgen.toml --crate cribbage-rs --output cribbage-rs.h WORKING_DIRECTORY ../cribbage-rs)

link_directories(cribbage-rs/target/debug)

add_executable(Cribbage cribbage.cpp)
target_link_libraries(Cribbage cribbage_rs)
