pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for i in 0..(input.len()) {
        input[i].iter().enumerate().filter(|&(_,v1)|{
            !input[i].iter().any(|v2| {
                v2>v1
            })
        }).for_each(|(j,v)|{
            let mut r = true;
            for k in 0..(input.len()){
                if input[k][j] < *v {
                    r = false;
                    break;
                }
            }
            if r{
                result.push((i,j))
            }
        });
    }
    result
}
