
/// GeoPOI 地图上某个点
pub struct GeoPOI{
    pub latitude: f64,
    pub longitude: f64,
    pub name: String,
}


/// GeoHash GeoHash对象
pub struct GeoHash{
    pub geo_hash_string: String,

    // 矩形区域内最小纬度, 最大纬度
    pub min_lat: f64,
    pub max_lat: f64,

    // 矩形区域内最大经度, 最大纬度
    pub min_lng: f64,
    pub max_lng: f64,
}



