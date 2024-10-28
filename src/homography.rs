use faer;
pub fn tag_homography(corners: &Vec<(f32, f32)>, border_bit: u8) -> faer::Mat<f32> {
    let source = vec![
        (-0.5, -0.5),
        (-0.5, border_bit as f32 - 0.5),
        (border_bit as f32 - 0.5, border_bit as f32 - 0.5),
        (border_bit as f32 - 0.5, -0.5),
    ];
    let mut mat_a = faer::Mat::<f32>::zeros(8, 9);
    for p in 0..4 {
        unsafe {
            mat_a.write_unchecked(p * 2, 0, source[p].0);
            mat_a.write_unchecked(p * 2, 1, source[p].1);
            mat_a.write_unchecked(p * 2, 2, 1.0);
            mat_a.write_unchecked(p * 2, 6, -1.0 * corners[p].0 * source[p].0);
            mat_a.write_unchecked(p * 2, 7, -1.0 * corners[p].0 * source[p].1);
            mat_a.write_unchecked(p * 2, 8, -1.0 * corners[p].0);
            mat_a.write_unchecked(p * 2 + 1, 3, source[p].0);
            mat_a.write_unchecked(p * 2 + 1, 4, source[p].1);
            mat_a.write_unchecked(p * 2 + 1, 5, 1.0);
            mat_a.write_unchecked(p * 2 + 1, 6, -1.0 * corners[p].1 * source[p].0);
            mat_a.write_unchecked(p * 2 + 1, 7, -1.0 * corners[p].1 * source[p].1);
            mat_a.write_unchecked(p * 2 + 1, 8, -1.0 * corners[p].1);
        }
    }
    // let svd = (mat_a.transpose()*mat_a.clone()).svd();
    let svd = mat_a.clone().svd();
    let h = svd.v().col(8);
    faer::mat![[h[0], h[1], h[2]], [h[3], h[4], h[5]], [h[6], h[7], h[8]],]
}
