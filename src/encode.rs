use crate::geo_hash;
use crate::geo_hash::GeoHash;


const GEO_BASE32: &[u8] = b"0123456789bcdefghjkmnpqrstuvwxyz";
const GEO_BITS:[i8; 5] = [16,8,4,2,1];

fn geo_hash_encode(poi: geo_hash::GeoPOI, precision: u8) -> geo_hash::GeoHash{

    let mut geoHashBytes: String = String::new();

    /// - bit: 标识当前循环ch字节长度
    /// - ch: geoHash其中的每个字符
    /// - length: geoHash的总长度标志
    /// - isEven: 经纬度切换标志
    let mut bits_index: u8 = 0;
    let mut geo_char_index:usize = 0;
    let mut length: u8 = 0;
    let mut is_even = true;

    /// mid: 中间值
    let mut mid = 0.0;
    /// 全图最小纬度, 最大纬度,
    let mut min_lat: f64 = -90.0;
    let mut max_lat: f64 = 90.0;

    /// 最小经度, 最大经度
    let mut min_lng: f64 = -180.0;
    let mut max_lng: f64 = 180.0;

    while length < precision{
        if is_even {
            // 偶数，经度
            mid = (min_lng + max_lng)/2.0;

        }else{
            // 奇数，纬度


        }

        is_even = !is_even;
        if bits_index < 4{
            bits_index = bits_index + 1;
        }else{
            geoHashBytes.push(GEO_BASE32[geo_char_index].as_char());
            length = length +1;
            bits_index = 0;
            geo_char = 0;
        }
    }


    let a = GEO_BASE32[1].clone();
    let b = GEO_BITS[2].clone();

    GeoHash{
        geo_hash_string: geoHashBytes,
        min_lat,
        max_lat,
        min_lng,
        max_lng
    }

}