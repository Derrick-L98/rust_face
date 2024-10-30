// // // This file is part of the open-source port of SeetaFace engine, which originally includes three modules:
// // //      SeetaFace Detection, SeetaFace Alignment, and SeetaFace Identification.
// // //
// // // This file is part of the SeetaFace Detection module, containing codes implementing the face detection method described in the following paper:
// // //
// // //      Funnel-structured cascade for multi-view face detection with alignment awareness,
// // //      Shuzhe Wu, Meina Kan, Zhenliang He, Shiguang Shan, Xilin Chen.
// // //      In Neurocomputing (under review)
// // //
// // // Copyright (C) 2016, Visual Information Processing and Learning (VIPL) group,
// // // Institute of Computing Technology, Chinese Academy of Sciences, Beijing, China.
// // //
// // // As an open-source face recognition engine: you can redistribute SeetaFace source codes
// // // and/or modify it under the terms of the BSD 2-Clause License.
// // //
// // // You should have received a copy of the BSD 2-Clause License along with the software.
// // // If not, see < https://opensource.org/licenses/BSD-2-Clause>.

// // use std::env::Args;
// // use std::time::{Duration, Instant};

// // use image::{DynamicImage, GrayImage, Rgb};
// // use imageproc::drawing::draw_hollow_rect_mut;
// // use imageproc::rect::Rect;

// // use rustface::{Detector, FaceInfo, ImageData};

// // const OUTPUT_FILE: &str = "test.png";

// // fn main() {
// //     let options = match Options::parse(std::env::args()) {
// //         Ok(options) => options,
// //         Err(message) => {
// //             println!("Failed to parse program arguments: {}", message);
// //             std::process::exit(1)
// //         }
// //     };

// //     let mut detector = match rustface::create_detector(options.model_path()) {
// //         Ok(detector) => detector,
// //         Err(error) => {
// //             println!("Failed to create detector: {}", error);
// //             std::process::exit(1)
// //         }
// //     };

// //     detector.set_min_face_size(20);
// //     detector.set_score_thresh(2.0);
// //     detector.set_pyramid_scale_factor(0.8);
// //     detector.set_slide_window_step(4, 4);

// //     let image: DynamicImage = match image::open(options.image_path()) {
// //         Ok(image) => image,
// //         Err(message) => {
// //             println!("Failed to read image: {}", message);
// //             std::process::exit(1)
// //         }
// //     };

// //     let mut rgb = image.to_rgb8();
// //     let faces = detect_faces(&mut *detector, &image.to_luma8());

// //     for face in faces {
// //         let bbox = face.bbox();
// //         let rect = Rect::at(bbox.x(), bbox.y()).of_size(bbox.width(), bbox.height());

// //         draw_hollow_rect_mut(&mut rgb, rect, Rgb([255, 0, 0]));
// //     }

// //     match rgb.save(OUTPUT_FILE) {
// //         Ok(_) => println!("Saved result to {}", OUTPUT_FILE),
// //         Err(message) => println!("Failed to save result to a file. Reason: {}", message),
// //     }
// // }

// // fn detect_faces(detector: &mut dyn Detector, gray: &GrayImage) -> Vec<FaceInfo> {
// //     let (width, height) = gray.dimensions();
// //     let image = ImageData::new(gray, width, height);
// //     let now = Instant::now();
// //     let faces = detector.detect(&image);
// //     println!(
// //         "Found {} faces in {} ms",
// //         faces.len(),
// //         get_millis(now.elapsed())
// //     );
// //     faces
// // }

// // fn get_millis(duration: Duration) -> u64 {
// //     duration.as_secs() * 1000u64 + u64::from(duration.subsec_millis())
// // }

// // struct Options {
// //     image_path: String,
// //     model_path: String,
// // }

// // impl Options {
// //     fn parse(args: Args) -> Result<Self, String> {
// //         let args: Vec<String> = args.into_iter().collect();
// //         if args.len() != 3 {
// //             return Err(format!("Usage: {} <model-path> <image-path>", args[0]));
// //         }

// //         let model_path = args[1].clone();
// //         let image_path = args[2].clone();

// //         Ok(Options {
// //             image_path,
// //             model_path,
// //         })
// //     }

// //     fn image_path(&self) -> &str {
// //         &self.image_path[..]
// //     }

// // use opencv::core::{Rect, Scalar};
// //     fn model_path(&self) -> &str {
// //         &self.model_path[..]
// //     }
// // }
use opencv::{
    core::{Size, Vector},
    highgui::{imshow, wait_key},
    imgcodecs::{imread, IMREAD_COLOR},
    imgproc::LineTypes::LINE_8,
    objdetect::CascadeClassifier,
    videoio::{VideoCapture, CAP_ANY},
    // imgproc::;
};
use opencv::{imgproc, prelude::*};

fn main() {
    // 初始化摄像头
    // let mut cam = VideoCapture::new(0, CAP_ANY).unwrap();
    // if !cam.is_opened().unwrap() {
    //     panic!("摄像头未打开！");
    // }

    // 加载人脸检测模型
    let mut face_cascade = CascadeClassifier::new("haarcascade_frontalface_default.xml")
        .expect("无法加载人脸检测模型");

    // 读取图像
    let mut img = imread("R-C.jpg", IMREAD_COLOR).expect("无法读取图像");

    // 将图像转换为灰度图像
    let mut gray = opencv::core::Mat::default();
    opencv::imgproc::cvt_color(&img, &mut gray, opencv::imgproc::COLOR_BGR2GRAY, 0).unwrap();

    // 进行人脸检测
    let mut faces = Vector::<opencv::core::Rect>::new();
    // let mut faces = Vec::new(); // 创建一个空的向量来存储检测到的人脸
    face_cascade
        .detect_multi_scale(
            &gray,
            &mut faces,
            1.1,
            3,
            0,
            Size::new(30, 30),
            Size::new(0, 0),
        )
        .unwrap();

    // 在图像上绘制人脸框
    for face in faces.iter() {
        // let point1 = opencv::core::Point::new(face.x, face.y);
        // let point2 = opencv::core::Point::new(face.x + face.width, face.y + face.height);
        // opencv::imgproc::rectangle(
        //     &mut img,
        //     point1,
        //     point2,
        //     opencv::core::Scalar::new(255.0, 0.0, 0.0, 0.0),
        //     2,
        //     opencv::imgproc::LINE_8,
        //     0,
        // )
        // .unwrap();
        imgproc::rectangle(
            &mut img,
            opencv::core::Rect::new(face.x, face.y, face.width, face.height),
            opencv::core::Scalar::new(0.0, 255.0, 0.0, 0.0), // 矩形颜色为绿色
            2,
            LINE_8 as i32,
            0,
        )
        .unwrap();
    }

    // 显示结果
    imshow("人脸检测", &img).unwrap();
    wait_key(0).unwrap();
}

// use opencv::{
//     core, highgui, imgproc, objdetect,
//     prelude::*,
//     videoio::{VideoCapture, CAP_ANY},
//     Result,
// };

// fn main() -> Result<()> {
//     // 初始化摄像头
//     let mut cam = VideoCapture::new(0, CAP_ANY)?;
//     if !cam.is_opened()? {
//         panic!("摄像头未打开！");
//     }

//     println!("开始识别人脸！");
//     // 加载人脸检测模型
//     let mut face_cascade =
//         objdetect::CascadeClassifier::new("haarcascade_frontalface_default.xml")?;

//     loop {
//         let mut frame = Mat::default();
//         cam.read(&mut frame)?;

//         // 转换为灰度图
//         let mut gray = Mat::default();
//         imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

//         // 检测人脸
//         let mut faces = opencv::types::VectorOfRect::new();
//         face_cascade
//             .detect_multi_scale(
//                 &gray,
//                 &mut faces,
//                 1.1,
//                 3,
//                 objdetect::CASCADE_SCALE_IMAGE,
//                 core::Size::new(30, 30),
//                 core::Size::new(30, 30),
//             )
//             .unwrap();

//         // 在检测到的人脸上绘制矩形
//         for face in faces {
//             imgproc::rectangle(
//                 &mut frame,
//                 face,
//                 core::Scalar::new(0.0, 255.0, 0.0, 0.0),
//                 2,
//                 8,
//                 0,
//             )?;
//         }

//         // 显示结果
//         highgui::imshow("人脸识别", &frame)?;
//         if highgui::wait_key(10)? > 0 {
//             break;
//         }
//     }

//     Ok(())
// }
