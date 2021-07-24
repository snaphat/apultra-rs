extern crate cty;
#[repr(C)]
pub struct apultra_stats
{
    pub num_literals:         cty::c_int,
    pub num_4bit_matches:     cty::c_int,
    pub num_7bit_matches:     cty::c_int,
    pub num_variable_matches: cty::c_int,
    pub num_rep_matches:      cty::c_int,
    pub num_eod:              cty::c_int,
    pub safe_dist:            cty::c_int,
    pub min_offset:           cty::c_int,
    pub max_offset:           cty::c_int,
    pub total_offsets:        cty::c_longlong,
    pub min_match_len:        cty::c_int,
    pub max_match_len:        cty::c_int,
    pub total_match_lens:     cty::c_int,
    pub min_rle1_len:         cty::c_int,
    pub max_rle1_len:         cty::c_int,
    pub total_rle1_lens:      cty::c_int,
    pub min_rle2_len:         cty::c_int,
    pub max_rle2_len:         cty::c_int,
    pub total_rle2_lens:      cty::c_int,
    pub commands_divisor:     cty::c_int,
    pub match_divisor:        cty::c_int,
    pub rle1_divisor:         cty::c_int,
    pub rle2_divisor:         cty::c_int,
}

impl Default for apultra_stats
{
    fn default() -> apultra_stats
    {
        apultra_stats {
            num_literals:         0,
            num_4bit_matches:     0,
            num_7bit_matches:     0,
            num_variable_matches: 0,
            num_rep_matches:      0,
            num_eod:              0,
            safe_dist:            0,
            min_offset:           0,
            max_offset:           0,
            total_offsets:        0,
            min_match_len:        0,
            max_match_len:        0,
            total_match_lens:     0,
            min_rle1_len:         0,
            max_rle1_len:         0,
            total_rle1_lens:      0,
            min_rle2_len:         0,
            max_rle2_len:         0,
            total_rle2_lens:      0,
            commands_divisor:     0,
            match_divisor:        0,
            rle1_divisor:         0,
            rle2_divisor:         0,
        }
    }
}

extern "C" {
    pub fn apultra_compress(
        pInputData: *const cty::c_uchar,
        pOutBuffer: *const cty::c_uchar,
        nInputSize: cty::size_t,
        nMaxOutBufferSize: cty::size_t,
        nFlags: cty::c_uint,
        nMaxWindowSize: cty::size_t,
        nDictionarySize: cty::size_t,
        progress: Option<extern "C" fn(cty::c_longlong, cty::c_longlong)>,
        pStats: Option<&mut apultra_stats>,
    ) -> cty::ssize_t;

    pub fn apultra_decompress(
        pInputData: *const cty::c_uchar,
        pOutBuffer: *const cty::c_uchar,
        nInputSize: cty::size_t,
        nMaxOutBufferSize: cty::size_t,
        nDictionarySize: cty::size_t,
        nFlags: cty::c_uint,
    ) -> cty::ssize_t;

    pub fn apultra_get_max_compressed_size(nInputSize: cty::size_t) -> cty::size_t;
    pub fn apultra_get_max_decompressed_size(
        pInputData: *const cty::c_uchar,
        nInputSize: cty::size_t,
        flags: cty::c_uint,
    ) -> cty::size_t;
}

#[test]
fn test_compress()
{
    let mut stats = apultra_stats::default();
    let input = b"1234567890";
    let window_size = 32;
    let mut output = vec![0u8; 40];
    extern "C" fn progress(a: cty::c_longlong, b: cty::c_longlong)
    {
        println!("{} {}", a, b);
    }

    let output_len = unsafe {
        apultra_compress(
            &input[0],
            &output[0],
            input.len(),
            output.len(),
            0,
            window_size,
            0,
            Some(progress),
            Some(&mut stats),
        )
    };
    assert_eq!(output_len, 13);
    output.resize(output_len as usize, 0);
    assert_eq!(output, [49, 0, 50, 51, 52, 53, 54, 55, 56, 57, 96, 48, 0]);
}

#[test]
fn test_compress_error()
{
    let mut stats = apultra_stats::default();
    let input = b"1234567890";
    let window_size = 32;
    let mut _output = vec![0u8; 2];
    extern "C" fn progress(a: cty::c_longlong, b: cty::c_longlong)
    {
        println!("{} {}", a, b);
    }

    let output_len = unsafe {
        apultra_compress(
            &input[0],
            &_output[0],
            input.len(),
            _output.len(),
            0,
            window_size,
            0,
            Some(progress),
            Some(&mut stats),
        )
    };
    assert_eq!(output_len, -1);
}

#[test]
fn test_decompress()
{
    let input = vec![49, 0, 50, 51, 52, 53, 54, 55, 56, 57, 96, 48, 0];
    let window_size = 32;
    let mut output = vec![0u8; 40];

    let output_len = unsafe {
        apultra_decompress(&input[0], &output[0], input.len(), output.len(), 0, window_size)
    };
    assert_eq!(output_len, 10);
    output.resize(output_len as usize, 0);
    assert_eq!(output, b"1234567890");
}

#[test]
fn test_decompress_error()
{
    let input = vec![45];
    let window_size = 0;
    let mut _output = vec![0u8; 40];

    let output_len = unsafe {
        apultra_decompress(&input[0], &_output[0], input.len(), _output.len(), 0, window_size)
    };
    assert_eq!(output_len, -1);
}

#[test]
fn test_get_max_compressed_size()
{
    unsafe {
        assert_eq!(apultra_get_max_compressed_size(0), 2);
        assert_eq!(apultra_get_max_compressed_size(1), 3);
        assert_eq!(apultra_get_max_compressed_size(2), 4);
        assert_eq!(apultra_get_max_compressed_size(3), 5);
    }
}

#[test]
fn test_get_max_decompressed_size()
{
    let input = vec![49, 0, 50, 51, 52, 53, 54, 55, 56, 57, 96, 48, 0];
    unsafe {
        assert_eq!(apultra_get_max_decompressed_size(input.as_ptr(), input.len(), 0), 10);
    }
}
