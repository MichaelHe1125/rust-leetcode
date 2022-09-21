// Largest product in a series
   
// Problem 8
// The four adjacent digits in the 1000-digit number that have the greatest product are 9 × 9 × 8 × 9 = 5832.

// 73167176531330624919225119674426574742355349194934
// 96983520312774506326239578318016984801869478851843
// 85861560789112949495459501737958331952853208805511
// 12540698747158523863050715693290963295227443043557
// 66896648950445244523161731856403098711121722383113
// 62229893423380308135336276614282806444486645238749
// 30358907296290491560440772390713810515859307960866
// 70172427121883998797908792274921901699720888093776
// 65727333001053367881220235421809751254540594752243
// 52584907711670556013604839586446706324415722155397
// 53697817977846174064955149290862569321978468622482
// 83972241375657056057490261407972968652414535100474
// 82166370484403199890008895243450658541227588666881
// 16427171479924442928230863465674813919123162824586
// 17866458359124566529476545682848912883142607690042
// 24219022671055626321111109370544217506941658960408
// 07198403850962455444362981230987879927244284909188
// 84580156166097919133875499200524063689912560717606
// 05886116467109405077541002256983155200055935729725
// 71636269561882670428252483600823257530420752963450

// Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. What is the value of this product?

pub struct Solution {}

const ADJACENT_LENGTHS: usize = 13;
impl Solution {
    pub fn largest_product_in_a_series(s: String) -> u64 {
        let mut result = 0u64;
        for i in 0..s.len() - ADJACENT_LENGTHS {
            let temp = &s[i..i+ADJACENT_LENGTHS];
            let value = temp
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .fold(1u64, |p, a| p*a as u64);
            if value > result { result = value; }
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_product_in_a_series_test() {
        assert_eq!(Solution::largest_product_in_a_series("7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055112540698747158523863050715693290963295227443043557668966489504452445231617318564030987111217223831136222989342338030813533627661428280644448664523874930358907296290491560440772390713810515859307960866701724271218839987979087922749219016997208880937766572733300105336788122023542180975125454059475224352584907711670556013604839586446706324415722155397536978179778461740649551492908625693219784686224828397224137565705605749026140797296865241453510047421663704844031998900088952434506585412275886668811642717147992444292823086346567481391912316282458617866458359124566529476545682848912883142607690042242190226710556263211111093705442175069416589604087198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450".to_owned()), 23514624000u64);
    }
}