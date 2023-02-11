#[derive(Debug,PartialEq,PartialOrd)]
pub struct Point2D {
    x:f64,
    y:f64,
}

impl Point2D {
    // construct the point (x, y)
    fn new(x:f64, y:f64) -> Self {
        Self { x, y }
    }
    
    // x-coordinate
    fn x(&self) -> f64 {
        self.x
    }

    // y-coordinate
    fn y(&self) -> f64 {
        self.y
    }
    
    // Euclidean distance between two points
    fn distance_to(&self, other:Point2D) -> f64 {
        todo!();
    }         
    public  double distanceSquaredTo(Point2D that)  // square of Euclidean distance between two points 
    public     int compareTo(Point2D that)          // for use in an ordered symbol table 
    public boolean equals(Object that)              // does this point equal that object? 
    public    void draw()                           // draw to standard draw 
    public  String toString()                       // string representation 
 }