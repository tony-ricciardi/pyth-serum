#if defined( __bpf__ ) || defined( SOL_TEST )
#include <solana_sdk.h>
#elif defined( __cplusplus )
#include <cstdint>
#else
#include <assert.h>  // c17 static_assert
#include <stdbool.h>
#include <stdint.h>
#endif
