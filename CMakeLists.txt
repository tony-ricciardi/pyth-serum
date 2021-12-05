cmake_minimum_required( VERSION 3.13 )

project( serum-pyth )
add_subdirectory( test-crank )

if( SOLANA )
  if( NOT PC )
    set( PC ${PROJECT_SOURCE_DIR}/../pyth-client )
  endif()
  include( scripts/bpf.cmake )
  add_bpf_lib( serum-pyth program/src/serum-pyth/serum-pyth.c )
  add_bpf_test( test_serum-pyth program/src/serum-pyth/test_serum-pyth.c )
  target_include_directories( serum-pyth PRIVATE ${PC}/program/src )
  target_include_directories( test_serum-pyth PRIVATE ${PC}/program/src )
endif()

# pyth-client build script runs ctest.
enable_testing()
