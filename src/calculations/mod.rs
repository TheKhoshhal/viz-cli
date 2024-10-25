pub fn shrink_vec(num_vec: &Vec<f32>, x: &usize) -> Vec<f32> {
    let mut prev: usize = 0;
    let mut index: usize;
    let mut new_vec: Vec<f32> = Vec::new();

    let mut arr: Vec<f32> = Vec::new();

    // shrink the main vector to fit the pixels of screen
    for i in 0..num_vec.len() {
        index = (i * x) / (num_vec.len() - 1);
        if index == prev {
            arr.push(num_vec[i].abs());
        } else {
            prev = index;
            let sum: f32 = arr.iter().sum();
            new_vec.push(sum / arr.len() as f32);

            arr = vec![num_vec[i].abs()];
        }
    }

    new_vec
}

pub fn calculate_percentage(new_vec: &Vec<f32>, y: &f32) -> Vec<u16> {
    let mut percentage: Vec<u16> = Vec::new();

    for i in new_vec {
        if i > &0.5 {
            percentage.push(y.floor() as u16);
        } else {
            let s = (y * (i / 0.5)).floor();
            percentage.push(s as u16);
        }
    }

    percentage
}
