use vehicle_payload::{ImageBoxCoordinates, LicensePlateStatus};

pub fn process_image_and_plate() {
    let coords = ImageBoxCoordinates { x: 100, y: 200 };
    let status = LicensePlateStatus { is_valid: true };
    let _ = (coords, status);
}
