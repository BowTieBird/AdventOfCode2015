use std::fs;

pub fn main(file_path: &str) {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut sum = 0;
    let mut bow = 0;
    for line in contents.split('\n') {
        if line == "" {
            break;
        }
        let dimensions: Vec<u32> = line.split('x').map(|x| x.parse::<u32>().unwrap()).collect();
        let volume: u32 = dimensions.iter().product();
        let mut areas: Vec<u32> = Vec::new();
        let mut perims: Vec<u32> = Vec::new();
        for i in 0..3 {
            areas.push(dimensions[i] * dimensions[(i+1)%3]);
            perims.push(dimensions[i] + dimensions[(i+1)%3]);
        }
        let area_sum: u32 = areas.iter().sum();
        let min_area = areas.iter().min().unwrap(); // Could be empty vector but we're lazy
        let min_perim = perims.iter().min().unwrap(); // Could be empty vector but we're lazy
        sum += 2* area_sum + min_area;
        bow += volume + 2*min_perim;
    }
    println!("{}", sum);
    println!("{}", bow);
}
