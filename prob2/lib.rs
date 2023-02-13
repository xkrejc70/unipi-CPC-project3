pub fn patriotic_selections(h: Vec<char>) -> usize {
    let mut r_cnt: usize = 0; // Red lights counter
    let mut w_cnt: usize = 0; // Red-White lights counter
    let mut g_cnt: usize = 0; // Red-White-Green lights counter
    let mut x_cnt: usize = 1; // Unassigned color counter
    let base: usize = 3; // 3 lights

    for i in 0..h.len() {
        match h[i] {
            'R' => r_cnt += x_cnt,
            'W' => w_cnt += r_cnt,
            'G' => g_cnt += w_cnt,
            'X' => {
                g_cnt = base * g_cnt + w_cnt;
                w_cnt = base * w_cnt + r_cnt;
                r_cnt = base * r_cnt + x_cnt;
                x_cnt = base * x_cnt;
            }
            _ => return 0,
        }
    }

    g_cnt
}
