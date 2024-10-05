#[macro_export]
macro_rules! classes {
  ($macro:ident $enum:ident { $($variant:ident [$($tag:ident)*])* }) => {
    classes! { ($) $macro $enum { $($variant [$($tag)*])*} }
  };
  (($d:tt) $macro:ident $enum:ident { $($variant:ident [$($tag:ident)*])* }) => {
    macro_rules! $macro {
      $($((#eval $variant {$tag} $d T:tt $d F:tt) => { $enum!$d T };)*)*
      (#eval $d v:ident {$d t:ident} $d T:tt $d F:tt) => { $enum!$d F };
      (#eval $d v:ident {not($d($d e:tt)*)} $d T:tt $d F:tt) => {
        $enum!(#eval $d v {$d($d e)*} $d F $d T)
       };
      (#eval $d v:ident {all($d a:ident $d(($d($d b:tt)*))? $d(, $d($d c:tt)*)?)} $d T:tt $d F:tt) => {
        $enum!(#eval $d v {$d a $d(($d($d b)*))?}
          [#eval $d v {all($d($d($d c)*)?)} $d T $d F]
          $d F
        )
      };
      (#eval $d v:ident {any($d a:ident $d(($d($d b:tt)*))? $d(, $d($d c:tt)*)?)} $d T:tt $d F:tt) => {
        $enum!(#eval $d v {$d a $d(($d($d b)*))?}
          $d T
          [#eval $d v {any($d($d($d c)*)?)} $d T $d F]
        )
      };
      (#eval $d v:ident {all()} $d T:tt $d F:tt) => { $enum!$d T };
      (#eval $d v:ident {any()} $d T:tt $d F:tt) => { $enum!$d F };
      (#filter [$d($d p:ident)*] [$d c:ident $d($d n:ident)*] $d e:tt) => {
        $enum!(#eval $d c $d e
          [#filter [$d($d p)* $d c] [$d($d n)*] $d e]
          [#filter [$d($d p)*] [$d($d n)*] $d e]
        )
      };
      (#filter [$d($d p:ident)*] [] $d e:tt) => {
        $d($enum::$d p { .. })|*
      };
      (#$d($d x:tt)*) => {};
      ($d($d e:tt)*) => {
        $enum!(#filter [] [$($variant)*] {$d($d e)*})
      };
    }
  };
}

#[cfg(test)]
#[allow(dead_code)]
mod test {
  enum Thing {
    Dog,
    Cat,
    Bird,
    Tree,
    Rock,
    Paper,
    Scissors,
  }

  classes! { thing Thing {
    Dog [animal alive organic]
    Cat [animal alive organic sharp]
    Bird [animal alive organic]
    Tree [alive plant organic]
    Rock []
    Paper [organic]
    Scissors [sharp]
  } }

  use thing as Thing;

  fn foo(x: Thing) {
    match x {
      Thing![not(any(sharp, all(organic, not(animal))))] => unreachable!(),
      Thing::Cat => todo!(),
      Thing::Tree => todo!(),
      Thing::Paper => todo!(),
      Thing::Scissors => todo!(),
    }
  }
}
