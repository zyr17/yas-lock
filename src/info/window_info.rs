use crate::common::PixelRectBound;
use crate::info::info::ScanInfo;

pub struct Rect(f64, f64, f64, f64); // top, right, bottom, left

pub struct WindowInfo {
    pub width: f64,
    pub height: f64,

    pub title_pos: Rect,
    pub main_stat_name_pos: Rect,
    pub main_stat_value_pos: Rect,
    pub level_pos: Rect,
    pub panel_pos: Rect,

    pub sub_stat1_pos: Rect,
    pub sub_stat2_pos: Rect,
    pub sub_stat3_pos: Rect,
    pub sub_stat4_pos: Rect,

    pub equip_pos: Rect,
    pub art_count_pos: Rect,

    pub art_width: f64,
    pub art_height: f64,
    pub art_gap_x: f64,
    pub art_gap_y: f64,

    pub art_row: usize,
    pub art_col: usize,

    pub left_margin: f64,
    pub top_margin: f64,

    pub flag_x: f64,
    pub flag_y: f64,

    pub star_x: f64,
    pub star_y: f64,

    pub lock_x: f64,
    pub lock_y: f64,

    pub pool_pos: Rect,
}

impl WindowInfo {
    pub fn to_scan_info(&self, h: f64, w: f64, left: i32, top: i32) -> ScanInfo {
        let convert_rect = |rect: &Rect| {
            let top = rect.0 / self.height * h;
            let right = rect.1 / self.width * w;
            let bottom = rect.2 / self.height * h;
            let left = rect.3 / self.width * w;

            PixelRectBound {
                left: left as i32,
                top: top as i32,
                right: right as i32,
                bottom: bottom as i32,
            }
        };

        let convert_x = |x: f64| x / self.width * w;

        let convert_y = |y: f64| y / self.height * h;

        ScanInfo {
            title_position: convert_rect(&self.title_pos),
            main_stat_name_position: convert_rect(&self.main_stat_name_pos),
            main_stat_value_position: convert_rect(&self.main_stat_value_pos),
            level_position: convert_rect(&self.level_pos),
            panel_position: convert_rect(&self.panel_pos),
            sub_stat1_position: convert_rect(&self.sub_stat1_pos),
            sub_stat2_position: convert_rect(&self.sub_stat2_pos),
            sub_stat3_position: convert_rect(&self.sub_stat3_pos),
            sub_stat4_position: convert_rect(&self.sub_stat4_pos),
            equip_position: convert_rect(&self.equip_pos),
            art_count_position: convert_rect(&self.art_count_pos),
            art_width: convert_x(self.art_width) as u32,
            art_height: convert_y(self.art_height) as u32,
            art_gap_x: convert_x(self.art_gap_x) as u32,
            art_gap_y: convert_y(self.art_gap_y) as u32,
            art_row: self.art_row as u32,
            art_col: self.art_col as u32,
            left_margin: convert_x(self.left_margin) as u32,
            top_margin: convert_y(self.top_margin) as u32,
            width: w as u32,
            height: h as u32,
            left,
            top,
            flag_x: convert_x(self.flag_x) as u32,
            flag_y: convert_y(self.flag_y) as u32,
            star_x: convert_x(self.star_x) as u32,
            star_y: convert_y(self.star_y) as u32,
            lock_x: convert_x(self.lock_x) as u32,
            lock_y: convert_y(self.lock_y) as u32,
            pool_position: convert_rect(&self.pool_pos),
        }
    }
}

pub const WINDOW_16_9: WindowInfo = WindowInfo {
    width: 1600.0,
    height: 900.0,

    title_pos: Rect(106.6, 1417.7, 139.6, 1111.8),
    main_stat_name_pos: Rect(224.3, 1253.9, 248.0, 1110.0),
    main_stat_value_pos: Rect(248.4, 1246.8, 286.8, 1110.0),
    level_pos: Rect(360.0, 1160.0, 378.0, 1117.0),
    panel_pos: Rect(100.0, 1500.0, 800.0, 1090.0),

    sub_stat1_pos: Rect(398.1, 1343.0, 427.3, 1130.2),
    sub_stat2_pos: Rect(427.3, 1343.0, 458.2, 1130.2),
    sub_stat3_pos: Rect(458.2, 1343.0, 490.9, 1130.2),
    sub_stat4_pos: Rect(490.9, 1343.0, 523.0, 1130.2),

    equip_pos: Rect(762.6, 1389.4, 787.8, 1154.9),
    art_count_pos: Rect(27.1, 1504.7, 52.9, 1314.9),

    art_width: 1055.0 - 953.0,
    art_height: 373.0 - 247.0,
    art_gap_x: 953.0 - 933.0,
    art_gap_y: 247.0 - 227.0,

    art_row: 5,
    art_col: 8,

    left_margin: 99.0,
    top_margin: 101.0,

    flag_x: 271.1,
    flag_y: 89.8,

    star_x: 1469.4,
    star_y: 123.9,

    lock_x: 1450.0,
    lock_y: 357.0,

    pool_pos: Rect(118.2, 1144.7 + 15.0, 510.3, 1144.7),
};

pub const WINDOW_8_5: WindowInfo = WindowInfo {
    width: 1440.0,
    height: 900.0,
    title_pos: Rect(96.0, 1268.9, 126.1, 1000.9),
    main_stat_name_pos: Rect(201.6, 1128.1, 223.9, 1000.3),
    main_stat_value_pos: Rect(225.5, 1128.1, 262.8, 1000.3),
    level_pos: Rect(324.0, 1043.0, 340.0, 1006.0),
    panel_pos: Rect(90.0, 1350.0, 810.0, 981.0),
    sub_stat1_pos: Rect(358.0, 1224.1, 384.1, 1016.2),
    sub_stat2_pos: Rect(384.1, 1224.1, 412.6, 1016.2),
    sub_stat3_pos: Rect(412.6, 1224.1, 440.5, 1016.2),
    sub_stat4_pos: Rect(440.5, 1224.1, 467.1, 1016.2),
    equip_pos: Rect(776.0, 1247.3, 800.6, 1041.3),
    art_count_pos: Rect(25.0, 1353.1, 46.8, 1182.8),
    art_width: 950.0 - 857.0,
    art_height: 204.0 - 91.0,
    art_gap_x: 857.0 - 840.0,
    art_gap_y: 222.0 - 204.0,
    art_row: 6,
    art_col: 8,
    left_margin: 89.0,
    top_margin: 91.0,
    flag_x: 245.9,
    flag_y: 82.1,
    star_x: 1321.3,
    star_y: 111.3,
    lock_x: 1305.0,
    lock_y: 322.0,
    pool_pos: Rect(103.6, 1025.8 + 15.0, 460.7, 1028.5),
};

pub const WINDOW_4_3: WindowInfo = WindowInfo {
    width: 1280.0,
    height: 960.0,
    title_pos: Rect(85.0, 1094.8, 111.7, 889.5),
    main_stat_name_pos: Rect(181.0, 998.0, 199.8, 889.5),
    main_stat_value_pos: Rect(199.8, 998.0, 233.4, 889.5),
    level_pos: Rect(288.0, 927.0, 302.0, 894.0),
    panel_pos: Rect(80.0, 1200.0, 880.0, 872.0),
    sub_stat1_pos: Rect(318.2, 1100.5, 342.3, 904.3),
    sub_stat2_pos: Rect(342.3, 1100.5, 369.4, 904.3),
    sub_stat3_pos: Rect(369.4, 1100.5, 395.3, 904.3),
    sub_stat4_pos: Rect(395.3, 1100.5, 420.6, 904.3),
    equip_pos: Rect(849.8, 1090.8, 870.1, 924.4),
    art_count_pos: Rect(22.9, 1202.3, 41.4, 1058.6),
    art_width: 844.0 - 762.0,
    art_height: 182.0 - 81.0,
    art_gap_x: 762.0 - 747.0,
    art_gap_y: 197.0 - 182.0,
    art_row: 7,
    art_col: 8,
    left_margin: 79.0,
    top_margin: 81.0,
    flag_x: 218.1,
    flag_y: 72.1,
    star_x: 1175.4,
    star_y: 95.8,
    lock_x: 1160.0,
    lock_y: 286.0,
    pool_pos: Rect(93.2, 912.7 + 15.0, 412.4, 912.7),
};
