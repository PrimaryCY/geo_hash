use crate::geo_hash;
use crate::geo_hash::GeoHash;


const GEO_BASE32_ARRAY: [char; 32] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
    'j', 'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
];

const GEO_BITS:[i8; 5] = [16,8,4,2,1];

fn geo_hash_encode(poi: geo_hash::GeoPOI, precision: u8) -> geo_hash::GeoHash{

    let mut geo_hash_bytes: String = String::new();

    // - bit: 标识当前循环ch字节长度
    // - ch: geoHash其中的每个字符
    // - length: geoHash的总长度标志
    // - isEven: 经纬度切换标志
    let mut geo_bits_index: u8 = 0;
    let mut geo_base32_array_index:usize = 0;
    let mut encode_length: u8 = 0;
    let mut is_even = true;

    // mid: 中间值
    let mut mid = 0.0;
    // 全图最小纬度, 最大纬度,
    let mut min_lat: f64 = -90.0;
    let mut max_lat: f64 = 90.0;

    // 最小经度, 最大经度
    let mut min_lng: f64 = -180.0;
    let mut max_lng: f64 = 180.0;

    while encode_length < precision{
        if is_even {
            // 偶数，经度
            mid = (min_lng + max_lng)/2.0;

        }else{
            // 奇数，纬度


        }

        is_even = !is_even;
        if geo_bits_index < 4{
            geo_bits_index = geo_bits_index + 1;
        }else{
            geo_hash_bytes.push(GEO_BASE32_ARRAY[geo_base32_array_index].clone());
            encode_length = encode_length +1;
            geo_bits_index = 0;
            geo_base32_array_index = 0;
        }
    }


    // let b = GEO_BITS[2].clone();

    GeoHash{
        geo_hash_string: geo_hash_bytes,
        min_lat,
        max_lat,
        min_lng,
        max_lng
    }

}
