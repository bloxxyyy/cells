pub struct Human {
    home: Position;
    places_to_visit: PlaceToVisit[];
}

pub struct PlaceToVisit {
    position: Position;
    visit_time_minuut: u32;
    leave_time_minuut: u32;
}

pub struct Position {
    x: i8;
    y: i8;
}