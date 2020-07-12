#[macro_export]
macro_rules! vec_comp {
    //========================I: single iterator case=============================
    // A: base case of iterator, any number of localized lets and finally a conditional
    // Ex. vec_comp![x; for x in 1..4; if x>1 ] >> [2, 3]
    ($f: expr; for $x: ident in $iterx:expr $(;let $s: ident = $v:expr)*; if $cond: expr $(;)*) => {{
        let mut myvec = Vec::new();
        let iter=$iterx;
        for $x in iter {
            $(let $s = $v;)*
            if $cond {
                myvec.push($f);
            };
        }
        myvec
    }};
    // B: A without a conditional; bootstraps A to be called with condition of true
    // Ex. vec_comp![x; for x in 1..4] >> [1, 2, 3]
    ($f: expr; for $x: ident in $iterx:expr $(;let $s: ident = $v:expr)* $(;)*) => {{
        vec_comp![$f; for $x in $iterx $(;let $s = $v)*; if true]
     }};
    //========================II: multi iterator case=============================
    // A: base case for multi iterator - let statement pairs WITH conditional
    // Ex. vec_comp![y*z; for x in 1..4; let y = x*x; for z in 1..y; let zz = 45; if x*zz>45] >> [4, 8, 12, 9, 18, 27, 36, 45, 54, 63, 72]
     ($f: expr; for $x: ident in $iterx:expr $(; let $s: ident = $v:expr)* $(; for $y: ident in $itery:expr $(; let $t: ident = $w:expr)*)+; if $cond: expr $(;)*) => {{
        // boilerplate uses x and looks nearly identical to A
        let mut myvec = Vec::new();
        let iter=$iterx;
        for $x in iter {
            $(let $s = $v;)*
            // recurse for y iterators and lets
            // calling case G until hit the single iterator case and then call F
            vec_comp![$f $(;for $y in $itery $(;let $t = $w)*)+; if $cond; myvec]
        }
        myvec
    }};
    // B: base case for multi iterator - let statement pairs WITHOUT conditional
    // simply wraps A as I-B did I-A
    // Ex. vec_comp![zz*z; for x in 1..4; let y = x*x; for z in 1..y; let zz = x*y; if y > 4] [27, 54, 81, 108, 135, 162, 189, 216]
    ($f: expr; for $x: ident in $iterx:expr $(; let $s: ident = $v:expr)* $(; for $y: ident in $itery:expr $(; let $t: ident = $w:expr)*)+ $(;)*) => {
        vec_comp![$f; for $x in $iterx $(; let $s = $v)* $(; for $y in $itery $(;let $t = $w)*)*; if true]
    };
    //========================III: w/ preallocated vectors=============================
    // A: given a user preallocated vector base case w/ conditional
    // Ex. let myvec = vec_comp![zz*z; for x in 1..4; let y = x*x; for z in 1..y; let zz = x*y; if y > 4; using Vec::new()];
    ($f: expr; for $x: ident in $iterx:expr $(;let $s: ident = $v:expr)* $(;for $y: ident in $itery:expr $(;let $t: ident = $w:expr)*)+; if $cond:expr; using $myvec: expr $(;)*) => {{
        let mut myvec=$myvec;
        let iter=$iterx;
        for $x in iter {
            $(let $s = $v;)*
            vec_comp![$f $(; for $y in $itery $(;let $t = $w)*)+; if $cond; myvec]
        }
        myvec
    }};
    // B: simply wraps A as I-B did I-A
    // Ex.     let mut myvec = vec![8, 6, 7, 5, 3, 0, 9];
    //         myvec = vec_comp![zz*z; for x in 1..4; let y = x*x; for z in 1..y; let zz = x*y; if true; using myvec];
    ($f: expr; for $x: ident in $iterx:expr $(;let $s: ident = $v:expr)* $(;for $y: ident in $itery:expr $(;let $t: ident = $w:expr)*)+; using $myvec: expr $(;)*) => {{
        let mut myvec=$myvec;
        let iter=$iterx;
        for $x in iter {
            $(let $s = $v;)*
            vec_comp![$f $(; for $y in $itery $(;let $t = $w)*)+; if true; myvec]
        }
        myvec
    }};
    //========================IV: used as helpers to above=============================
    // A: iterator helper base case (innermost nested loop i.e. last iterator after expanding multi iterator scenario)
    // used for recursive expansion of nested for loops once number of iterators in macro hits 1
    ($f: expr; for $x: ident in $iterx:expr $(;let $s: ident = $v:expr)*; if $cond: expr; $myvec: ident $(;)*) => {{
        let iter=$iterx;
        for $x in iter {
            $(let $s = $v;)*
            if $cond {
                $myvec.push($f);
            };
        }
    }};
    // B:  helper used to build nesting in multi iterator scenario. only called with 2+ iterators e.g. for ... in ...
    // Ex. let mut myvec = Vec::new()
    ($f: expr; for $x: ident in $iterx:expr $(;let $s: ident = $v:expr)* $(;for $y: ident in $itery:expr $(;let $t: ident = $w:expr)*)+; if $cond: expr; $myvec: ident $(;)*) => {{
        let iter=$iterx;
        for $x in iter {
            $(let $s = $v;)*
            vec_comp![$f $(; for $y in $itery $(;let $t = $w;)*)+; if $cond; $myvec]
        }
    }};
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_1itr() {
        assert_eq!(vec_comp![x; for x in 1..4], vec![1, 2, 3])
    }
    #[test]
    fn test_1itr_cond() {
        assert_eq!(vec_comp![x; for x in 1..4; if x>1], vec![2, 3])
    }
    #[test]
    fn test_1itr_1decl_cond() {
        assert_eq!(
            vec_comp![y; for x in 1..4; let y=x*x+4; if x>1],
            vec![8, 13]
        )
    }
    #[test]
    fn test_1itr_1decl() {
        assert_eq!(vec_comp![y; for x in 1..4; let y=x*x+4], vec![5, 8, 13])
    }
    #[test]
    fn test_1itr_2decl_cond() {
        assert_eq!(
            vec_comp![y+z; for x in 1..4; let y=x*x+4; let z = 3*y+x; if z>20],
            vec![34, 55]
        )
    }
    #[test]
    fn test_2itr_3decl_cond() {
        assert_eq!(
            vec_comp![y+zz*z; for x in 1..4; let y=x*x+4; let z = 3*y+x; for yy in 1..10; let zz= yy+1; if yy<3 && x>1],
            vec![60, 86, 97, 139]
        )
    }
    #[test]
    fn test_2itr_3decl_cond_myvec() {
        let myvec = vec![8, 6, 7, 5, 3, 0, 9];
        assert_eq!(
            vec_comp![y+zz*z; for x in 1..4; let y=x*x+4; let z = 3*y+x; for yy in 1..10; let zz= yy+1; if yy<3 && x>1; using myvec],
            vec![8, 6, 7, 5, 3, 0, 9, 60, 86, 97, 139]
        )
    }
}