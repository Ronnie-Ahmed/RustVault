use std::{collections::HashMap, iter::Cycle};
pub fn convert(s:String,num_rows:i32)->String{
   
    // let  temp_c:Vec<char>=s.chars().collect();
    // let mut temp_s=String::new();
    // let mut hmap=HashMap::new();
    // let mut temp=0;
    // for i in 1..num_rows+1{
    //     let n=(2*num_rows) -2;
    //     if i==1 || i==num_rows{
    //         hmap.insert(i, n);
    //         continue;
    //     }
    //     hmap.insert(i,n-i-temp);
    //     temp+=1;
    // }
    // for i in 1..num_rows+1{
    //     let  pos: &i32=hmap.get(&(i as i32)).unwrap();
    //     let mut j=i as usize;
        
    //     while j <= temp_c.len(){
    //         temp_s.push(temp_c[j-1]);
    //         j+=*pos as usize;            
    //     };
       
    // }



    // println!("{:?}",hmap)
     if num_rows<=1{
        return s
    }
    let  temp_c:Vec<char>=s.chars().collect();
    let mut temp_value=Vec::new();
    let mut position:HashMap<i32,Vec<usize>>=HashMap::new();
    let mut final_string=String::new();
    let mut temp=Vec::new();
    let leng=temp_c.len();
    let min=1;
    let max=num_rows;
    for i in 1..=num_rows{
        temp.push(i);
    }
    let pattern=(min..=max).chain(((min+1)..max).rev()).cycle().take(leng);
    for num in pattern{
        temp_value.push(num);
    }

    for (index,num) in temp_value.iter().enumerate(){
        position.entry(*num).or_default().push(index)
    }
    for num in temp{
        if let Some(pos) =position.get(&num){
            for n in pos{
                final_string.push(temp_c[*n]);
            }
        } 
    };


    println!("{:?}",final_string);
    final_string


}

fn main() {
    convert("PAYPALISHIRING".to_string(), 4);
}
//  PAYPALISHIRING
//  PINALIGYAIHRNPI
//  PINALSIGYAHRPI