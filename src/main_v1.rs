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
    let mut img = imread("face_sr.png", IMREAD_COLOR).expect("无法读取图像");

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
