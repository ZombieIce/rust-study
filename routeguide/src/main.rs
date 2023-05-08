use futures_core::Stream;
use std::hash::{Hash, Hasher};
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use routeguid::route_guide_server::{RouteGuide, RouteGuideServer};
use routeguid::{Feature, Point, Rectangle, RouteNote, RouteSummary};
pub mod routeguid {
    tonic::include_proto!("routeguide");
}

#[derive(Debug)]
pub struct RouteGuideService {
    features: Arc<Vec<Feature>>,
}

#[tonic::async_trait]
impl RouteGuide for RouteGuideService {
    async fn get_feature(&self, request: Request<Point>) -> Result<Response<Feature>, Status> {
        for feature in &self.features[..] {
            if feature.location.as_ref() == Some(request.get_ref()) {
                return Ok(Response::new(feature.clone()));
            }
        }
        Ok(Response::new(Feature::default()))
    }

    type ListFeaturesStream = ReceiverStream<Result<Feature, Status>>;

    async fn list_features(
        &self,
        request: Request<Rectangle>,
    ) -> Result<Response<Self::ListFeaturesStream>, Status> {
        let (mut tx, rx) = mpsc::channel(4);
        let features = self.features.clone();

        tokio::spawn(async move {
            for feature in &features[..] {
                if in_range(feature.location.as_ref().unwrap(), request.get_ref()) {
                    tx.send(Ok(feature.clone())).await.unwrap();
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn record_route(
        &self,
        request: Request<tonic::Streaming<Point>>,
    ) -> Result<Response<RouteSummary>, Status> {
        let mut _stream = request.into_inner();
        let mut _summary = RouteSummary::default();
        unimplemented!()


    }

    type RouteChatStream = Pin<Box<dyn Stream<Item = Result<RouteNote, Status>> + Send + 'static>>;
    async fn route_chat(
        &self,
        _request: Request<tonic::Streaming<RouteNote>>,
    ) -> Result<Response<Self::RouteChatStream>, Status> {
        unimplemented!()
    }
}

fn main() {
    println!("Hello, world!");
}

fn in_range(point: &Point, rect: &Rectangle) -> bool {
    use std::cmp;

    let lo = rect.lo.as_ref().unwrap();
    let hi = rect.hi.as_ref().unwrap();

    let left = cmp::min(lo.longtitude, hi.longtitude);
    let right = cmp::max(lo.longtitude, hi.longtitude);
    let top = cmp::max(point.latitude, hi.latitude);
    let bottom = cmp::min(lo.latitude, hi.latitude);

    point.longtitude >= left
        && point.longtitude <= right
        && point.latitude >= bottom
        && point.latitude <= top
}

impl Hash for Point {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.latitude.hash(state);
        self.longtitude.hash(state);
    }
}

impl Eq for Point {}

fn calc_distance(p1: &Point, p2: &Point) -> i32 {
    const CORD_FACTOR: f64 = 1e7;
    const R: f64 = 6_371_000.0;

    let lat1 = p1.latitude as f64 / CORD_FACTOR;
    let lat2 = p2.latitude as f64 / CORD_FACTOR;
    let lng1 = p1.longtitude as f64 / CORD_FACTOR;
    let lng2 = p2.longtitude as f64 / CORD_FACTOR;

    let lat_rad1 = lat1.to_radians();
    let lat_rad2 = lat2.to_radians();

    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lng = (lng2 - lng1).to_radians();

    let a = (delta_lat / 2f64).sin() * (delta_lat / 2f64).sin()
        + (lat_rad1).cos() * (lat_rad2).cos() * (delta_lng / 2f64).sin() * (delta_lng / 2f64).sin();
    let c = 2f64 * a.sqrt().atan2((1f64 - a).sqrt());
    (R * c) as i32
}
