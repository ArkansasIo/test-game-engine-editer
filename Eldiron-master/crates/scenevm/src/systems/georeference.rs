use vek::Vec3;

/// WGS84 ellipsoid constants.
const WGS84_A: f64 = 6_378_137.0; // semi-major axis (meters)
const WGS84_F: f64 = 1.0 / 298.257_223_563;
const WGS84_E2: f64 = WGS84_F * (2.0 - WGS84_F); // eccentricity^2

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeodeticCoord {
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub height_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EcefCoord {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnuCoord {
    pub east_m: f64,
    pub north_m: f64,
    pub up_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeoOrigin {
    pub geodetic: GeodeticCoord,
    ecef: EcefCoord,
    sin_lat: f64,
    cos_lat: f64,
    sin_lon: f64,
    cos_lon: f64,
}

impl GeoOrigin {
    pub fn new(geodetic: GeodeticCoord) -> Self {
        let ecef = geodetic_to_ecef(geodetic);
        let lat = geodetic.lat_deg.to_radians();
        let lon = geodetic.lon_deg.to_radians();
        Self {
            geodetic,
            ecef,
            sin_lat: lat.sin(),
            cos_lat: lat.cos(),
            sin_lon: lon.sin(),
            cos_lon: lon.cos(),
        }
    }
}

impl Default for GeoOrigin {
    fn default() -> Self {
        Self::new(GeodeticCoord {
            lat_deg: 0.0,
            lon_deg: 0.0,
            height_m: 0.0,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GeoChunkMapper {
    pub meters_per_chunk: f64,
}

impl GeoChunkMapper {
    pub fn new(meters_per_chunk: f64) -> Self {
        Self {
            meters_per_chunk: meters_per_chunk.max(1.0),
        }
    }

    pub fn enu_to_chunk(&self, enu: EnuCoord) -> (i32, i32, i32) {
        (
            floor_div(enu.east_m, self.meters_per_chunk),
            floor_div(enu.north_m, self.meters_per_chunk),
            floor_div(enu.up_m, self.meters_per_chunk),
        )
    }

    pub fn chunk_origin_enu(&self, chunk: (i32, i32, i32)) -> EnuCoord {
        EnuCoord {
            east_m: chunk.0 as f64 * self.meters_per_chunk,
            north_m: chunk.1 as f64 * self.meters_per_chunk,
            up_m: chunk.2 as f64 * self.meters_per_chunk,
        }
    }
}

impl Default for GeoChunkMapper {
    fn default() -> Self {
        Self::new(256.0)
    }
}

#[derive(Debug, Clone, Default)]
pub struct GeoReferenceSystem {
    pub origin: GeoOrigin,
    pub chunk_mapper: GeoChunkMapper,
}

impl GeoReferenceSystem {
    pub fn set_origin(&mut self, geodetic: GeodeticCoord) {
        self.origin = GeoOrigin::new(geodetic);
    }

    pub fn geodetic_to_local_enu(&self, geodetic: GeodeticCoord) -> EnuCoord {
        let ecef = geodetic_to_ecef(geodetic);
        ecef_to_enu(ecef, self.origin)
    }

    pub fn local_enu_to_geodetic(&self, enu: EnuCoord) -> GeodeticCoord {
        let ecef = enu_to_ecef(enu, self.origin);
        ecef_to_geodetic(ecef)
    }

    /// Convert ENU (meters) into a local f32 vector suitable for GPU-side scene chunks.
    pub fn enu_to_local_vec3f(&self, enu: EnuCoord) -> Vec3<f32> {
        Vec3::new(enu.east_m as f32, enu.north_m as f32, enu.up_m as f32)
    }

    pub fn geodetic_to_chunk(&self, geodetic: GeodeticCoord) -> (i32, i32, i32) {
        self.chunk_mapper
            .enu_to_chunk(self.geodetic_to_local_enu(geodetic))
    }
}

pub fn geodetic_to_ecef(g: GeodeticCoord) -> EcefCoord {
    let lat = g.lat_deg.to_radians();
    let lon = g.lon_deg.to_radians();
    let sin_lat = lat.sin();
    let cos_lat = lat.cos();
    let sin_lon = lon.sin();
    let cos_lon = lon.cos();
    let n = WGS84_A / (1.0 - WGS84_E2 * sin_lat * sin_lat).sqrt();
    EcefCoord {
        x: (n + g.height_m) * cos_lat * cos_lon,
        y: (n + g.height_m) * cos_lat * sin_lon,
        z: (n * (1.0 - WGS84_E2) + g.height_m) * sin_lat,
    }
}

pub fn ecef_to_geodetic(e: EcefCoord) -> GeodeticCoord {
    let lon = e.y.atan2(e.x);
    let p = (e.x * e.x + e.y * e.y).sqrt();

    // Iterative latitude solve (Bowring-like approach).
    let mut lat = (e.z / p.max(1e-12)).atan();
    for _ in 0..7 {
        let sin_lat = lat.sin();
        let n = WGS84_A / (1.0 - WGS84_E2 * sin_lat * sin_lat).sqrt();
        lat = (e.z + WGS84_E2 * n * sin_lat).atan2(p);
    }

    let sin_lat = lat.sin();
    let n = WGS84_A / (1.0 - WGS84_E2 * sin_lat * sin_lat).sqrt();
    let h = p / lat.cos().max(1e-12) - n;
    GeodeticCoord {
        lat_deg: lat.to_degrees(),
        lon_deg: lon.to_degrees(),
        height_m: h,
    }
}

pub fn ecef_to_enu(ecef: EcefCoord, origin: GeoOrigin) -> EnuCoord {
    let dx = ecef.x - origin.ecef.x;
    let dy = ecef.y - origin.ecef.y;
    let dz = ecef.z - origin.ecef.z;
    EnuCoord {
        east_m: -origin.sin_lon * dx + origin.cos_lon * dy,
        north_m: -origin.sin_lat * origin.cos_lon * dx - origin.sin_lat * origin.sin_lon * dy
            + origin.cos_lat * dz,
        up_m: origin.cos_lat * origin.cos_lon * dx
            + origin.cos_lat * origin.sin_lon * dy
            + origin.sin_lat * dz,
    }
}

pub fn enu_to_ecef(enu: EnuCoord, origin: GeoOrigin) -> EcefCoord {
    let dx = -origin.sin_lon * enu.east_m - origin.sin_lat * origin.cos_lon * enu.north_m
        + origin.cos_lat * origin.cos_lon * enu.up_m;
    let dy = origin.cos_lon * enu.east_m - origin.sin_lat * origin.sin_lon * enu.north_m
        + origin.cos_lat * origin.sin_lon * enu.up_m;
    let dz = origin.cos_lat * enu.north_m + origin.sin_lat * enu.up_m;
    EcefCoord {
        x: origin.ecef.x + dx,
        y: origin.ecef.y + dy,
        z: origin.ecef.z + dz,
    }
}

fn floor_div(value_m: f64, chunk_m: f64) -> i32 {
    (value_m / chunk_m).floor() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_geodetic_ecef() {
        let g = GeodeticCoord {
            lat_deg: 47.6205,
            lon_deg: -122.3493,
            height_m: 120.0,
        };
        let e = geodetic_to_ecef(g);
        let g2 = ecef_to_geodetic(e);
        assert!((g.lat_deg - g2.lat_deg).abs() < 1e-6);
        assert!((g.lon_deg - g2.lon_deg).abs() < 1e-6);
        assert!((g.height_m - g2.height_m).abs() < 1e-3);
    }

    #[test]
    fn roundtrip_enu() {
        let origin = GeoOrigin::new(GeodeticCoord {
            lat_deg: 35.0,
            lon_deg: -90.0,
            height_m: 20.0,
        });
        let enu = EnuCoord {
            east_m: 123.4,
            north_m: -56.7,
            up_m: 8.9,
        };
        let ecef = enu_to_ecef(enu, origin);
        let enu2 = ecef_to_enu(ecef, origin);
        assert!((enu.east_m - enu2.east_m).abs() < 1e-6);
        assert!((enu.north_m - enu2.north_m).abs() < 1e-6);
        assert!((enu.up_m - enu2.up_m).abs() < 1e-6);
    }
}
