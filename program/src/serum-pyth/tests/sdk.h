#pragma once

#include <solana_sdk.h>

// Aligned with errors in solana_sdk.h
static const char* SP_SOL_ERR_MSGS[] = {
  "SUCCESS",
  "CUSTOM_ZERO",
  "INVALID_ARGUMENT",
  "INVALID_INSTRUCTION_DATA",
  "INVALID_ACCOUNT_DATA",
  "ACCOUNT_DATA_TOO_SMALL",
  "INSUFFICIENT_FUNDS",
  "INCORRECT_PROGRAM_ID",
  "MISSING_REQUIRED_SIGNATURES",
  "ACCOUNT_ALREADY_INITIALIZED",
  "UNINITIALIZED_ACCOUNT",
  "NOT_ENOUGH_ACCOUNT_KEYS",
  "ACCOUNT_BORROW_FAILED",
  "MAX_SEED_LENGTH_EXCEEDED",
  "INVALID_SEEDS",
};

// Inverse of TO_BUILTIN(), for reading error codes in assertions.
// sp_error_idx( ERROR_INVALID_ARGUMENT ) == 2 -> "INVALID_ARGUMENT"
static inline uint64_t sp_error_idx( const uint64_t err )
{
  return ( err >> 32 ) & ( ( 1ul << 32 ) - 1 );
}

static inline const char* sp_error_msg( const uint64_t err )
{
  const uint64_t idx = sp_error_idx( err );
  return (
    idx < SOL_ARRAY_SIZE( SP_SOL_ERR_MSGS )
    ? SP_SOL_ERR_MSGS[ idx ]
    : "UNKNOWN_ERROR"
  );
}

static uint8_t* sp_push_u64( uint8_t* const buf, const uint64_t val )
{
  *( uint64_t* )buf = val;
  return buf + sizeof( val );
}

static uint8_t* sp_push_u8( uint8_t* const buf, const uint8_t val )
{
  *( uint8_t* )buf = val;
  return buf + sizeof( val );
}

static uint8_t* sp_push_key( uint8_t* const buf, const SolPubkey *const key )
{
  *( SolPubkey* )buf = *key;
  return buf + sizeof( *key );
}

static uint8_t* sp_push_data(
  uint8_t* buf,
  const uint8_t* const data,
  const uint64_t data_len
)
{
  buf = sp_push_u64( buf, data_len );
  sol_memcpy( buf, data, ( int )data_len );
  return buf + data_len;
}

// Max length needed by sol_deserialize for given params.
static uint64_t sp_input_len( const SolParameters* const params )
{
  uint64_t len = sizeof( params->ka_num );
  for ( uint64_t i = 0; i < params->ka_num; ++i ) {
    const SolAccountInfo *const acc = params->ka + i;
    len += sizeof( uint8_t ); // dup_info
    len += sizeof( acc->is_signer );
    len += sizeof( acc->is_writable );
    len += sizeof( acc->executable );
    len += 4; // padding
    len += sizeof( *acc->key );
    len += sizeof( *acc->owner );
    len += sizeof( *acc->lamports );
    len += sizeof( acc->data_len );
    len += acc->data_len;
    len += MAX_PERMITTED_DATA_INCREASE;
    len += 7; // max padding
    len += sizeof( acc->rent_epoch );
  }
  len += sizeof( params->data_len );
  len += params->data_len;
  len += sizeof( *params->program_id );
  return len;
}

// Inverse of sol_deserialize.
// buf should have a min length of sp_input_len( params ).
static uint8_t* sp_serialize( const SolParameters *const params, uint8_t *buf )
{
  buf = sp_push_u64( buf, params->ka_num );
  for ( uint64_t i = 0; i < params->ka_num; i++ ) {
    const SolAccountInfo* const acc = params->ka + i;
    buf = sp_push_u8( buf, UINT8_MAX ); // dup_info
    buf = sp_push_u8( buf, acc->is_signer );
    buf = sp_push_u8( buf, acc->is_writable );
    buf = sp_push_u8( buf, acc->executable );
    buf += 4; // padding
    buf = sp_push_key( buf, acc->key );
    buf = sp_push_key( buf, acc->owner );
    buf = sp_push_u64( buf, *acc->lamports );
    buf = sp_push_data( buf, acc->data, acc->data_len );
    buf += MAX_PERMITTED_DATA_INCREASE;
    buf = ( uint8_t* )( ( ( uint64_t )buf + 7 ) & ~7ul ); // padding
    buf = sp_push_u64( buf, acc->rent_epoch );
  }
  buf = sp_push_data( buf, params->data, params->data_len );
  buf = sp_push_key( buf, params->program_id );
  return buf;
}
