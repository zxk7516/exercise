pub fn build_proverb(list: Vec<&str>) -> String {
    let mut r = Vec::new();
    if list.len()>0{
        for i in 0..list.len()-1{
            r.push( format!("For want of a {} the {} was lost.",list[i],list[i+1])  );            
        }
        r.push( format!("And all for the want of a {}.",list[0])  );
    }

    if r.len()>0{
        r.join("\n")
    }else{
        String::new()
    }
    
}
