const UNDEFINED: i32 = -1;

// Optimized brute force method, without calculating redundant cases
pub fn opt_method(num_cities: usize, num_days: usize, cities: Vec<Vec<usize>>) -> usize {
    let num_attrs: usize = 0;
    let current_city: usize = 0;
    let num_days_left: usize = num_days;
    let mut array: Vec<Vec<i32>> = vec![vec![UNDEFINED; num_days + 1]; num_cities];

    method_2(
        num_days_left,
        num_attrs,
        &cities,
        num_cities,
        current_city,
        &mut array,
    )
}

fn method_2(
    num_days_left: usize,
    num_a: usize,
    cities: &Vec<Vec<usize>>,
    num_cities: usize,
    current_city: usize,
    array: &mut Vec<Vec<i32>>,
) -> usize {
    let mut local_max;
    let mut max = 0;

    // No cities left
    if current_city == num_cities {
        // Return current number of visited attraction as max
        return num_a;
    }

    // No days left
    if num_days_left == 0 {
        // Return current number of visited attraction as max
        return num_a;
    }

    if array[current_city][num_days_left] != UNDEFINED {
        return array[current_city][num_days_left] as usize + num_a;
    }

    // For all remaining days
    for j in 0..num_days_left + 1 {
        let mut num_attrs = num_a;
        if j > 0 {
            num_attrs += cities[current_city][j - 1];
        }

        local_max = method_2(
            num_days_left - j,
            num_attrs,
            cities,
            num_cities,
            current_city + 1,
            array,
        );

        // Find and return the maximum of all options
        if local_max > max {
            max = local_max;
        }
    }

    // Save result to array (by current city and number of remaining days) so it can be used after if needed
    array[current_city][num_days_left] = (max - num_a) as i32;

    max
}

// Brute force method using recursion, calculating all possible options
pub fn brute_force_method(num_cities: usize, num_days: usize, cities: Vec<Vec<usize>>) -> usize {
    let num_attrs: usize = 0;
    let current_city: usize = 0;
    let num_days_left: usize = num_days;

    method_1(num_days_left, num_attrs, &cities, num_cities, current_city)
}

fn method_1(
    num_days_left: usize,
    num_a: usize,
    cities: &Vec<Vec<usize>>,
    num_cities: usize,
    current_city: usize,
) -> usize {
    let mut local_max;
    let mut max = 0;

    // No city left
    if current_city == num_cities {
        // Return current number of attraction as max
        return num_a;
    }

    // No days left
    if num_days_left == 0 {
        // Return number of attraction as max
        return num_a;
    }

    // For all remaining days
    for j in 0..num_days_left + 1 {
        // Update variables
        let num_days_left = num_days_left - j;
        let current_city = current_city + 1;
        let mut num_attrs = num_a;

        if j > 0 {
            num_attrs += cities[current_city - 1][j - 1];
        }

        local_max = method_1(num_days_left, num_attrs, cities, num_cities, current_city);

        // Find and return the maximum of all options
        if local_max > max {
            max = local_max;
        }
    }
    max
}
