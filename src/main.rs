// use opencv::{
//     core, imgcodecs, imgproc, objdetect,
//     prelude::*,
//     types,
//     videoio::{VideoCapture, CAP_ANY},
//     Result,
// };

// fn detect_faces(
//     face_cascade: &mut objdetect::CascadeClassifier,
//     frame: &Mat,
// ) -> Result<types::VectorOfRect> {
//     let mut gray = Mat::default();
//     imgproc::cvt_color(frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
//     let mut faces = types::VectorOfRect::new();
//     face_cascade.detect_multi_scale(
//         &gray,
//         &mut faces,
//         1.1,
//         3,
//         objdetect::CASCADE_SCALE_IMAGE,
//         core::Size::new(30, 30),
//         core::Size::new(30, 30),
//     )?;
//     Ok(faces)
// }

// fn main() -> Result<()> {
//     // 加载人脸检测模型
//     let mut face_cascade =
//         objdetect::CascadeClassifier::new("haarcascade_frontalface_default.xml")?;

//     // 记录人脸的特征（示例使用文件保存特征）
//     let mut known_faces = Vec::new();

//     // 读取记录人脸照片
//     let image = imgcodecs::imread("R-C.jpg", imgcodecs::IMREAD_COLOR)?;
//     let faces = detect_faces(&mut face_cascade, &image)?;

//     for face in faces {
//         // 这里可以提取和保存人脸特征，使用人脸识别算法（例如 Eigenfaces, Fisherfaces, LBPH 等）
//         // 假设将特征保存到 known_faces 向量（此处用简单示例表示）
//         known_faces.push(face);
//     }

//     // 初始化摄像头
//     let mut cam = VideoCapture::new(0, CAP_ANY)?;
//     if !cam.is_opened()? {
//         panic!("摄像头未打开！");
//     }

//     loop {
//         let mut frame = Mat::default();
//         cam.read(&mut frame)?;

//         // 检测当前画面中的人脸
//         let faces = detect_faces(&mut face_cascade, &frame)?;

//         for face in faces {
//             imgproc::rectangle(
//                 &mut frame,
//                 face,
//                 core::Scalar::new(0.0, 255.0, 0.0, 0.0),
//                 2,
//                 8,
//                 0,
//             )?;

//             // 这里可以进行人脸匹配，比较当前 face 和 known_faces 中的特征
//             // 假设 match_faces 是一个函数，用于比较特征并判断是否匹配
//             // if match_faces(&face, &known_faces) {
//             //     println!("匹配成功！");
//             // }
//         }

//         // 显示结果
//         opencv::highgui::imshow("人脸识别", &frame)?;
//         if opencv::highgui::wait_key(10)? > 0 {
//             break;
//         }
//     }

//     Ok(())
// }

// // rust 基于opencv实现人脸识别功能,要求可以根据照片识别人脸是否匹配，也可以通过扫描记录人脸进行识别

// // 假设你已经有一组已知的人脸图像（用于记录），我们可以使用一些简单的方法来提取和匹配它们。这里是一个基础的代码示例：

// // 人脸匹配的实现：

// // 人脸识别通常涉及到特征提取和相似度比较，这里仅提供基础的框架。你可以使用 Eigenfaces、Fisherfaces 或者 LBPH 方法来实现人脸识别。这些方法会涉及到训练模型，你需要准备好数据集。

// // 注意事项：
// // 使用 OpenCV 进行人脸匹配需要选择适合的算法并实现，代码中仅描述了整体框架。
// // 请确保 haarcascade_frontalface_default.xml 文件的路径正确。
// // 需要处理匹配方法的具体实现，可能会涉及到更复杂的线性代数运算。
// // 确保已安装 OpenCV，并配置好 Rust 的绑定。
// // 这是一个初步的思路和实现示例，你可以根据项目需求进行扩展和细化。

use opencv::{
    core::{self, Rect},
    highgui, imgcodecs,
    imgproc::{self, LINE_AA},
    objdetect,
    prelude::*,
    types,
    videoio::{VideoCapture, CAP_ANY},
    Result,
};

fn detect_faces(
    face_cascade: &mut objdetect::CascadeClassifier,
    frame: &Mat,
) -> Result<core::Vector<core::Rect>> {
    //转换图像为灰度图
    let mut gray = Mat::default();
    imgproc::cvt_color(frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
    println!("detect_faces： {:?}", gray.size());
    // 检测人脸
    let mut faces = core::Vector::<core::Rect>::new();
    face_cascade.detect_multi_scale(
        &gray,
        &mut faces,
        1.1,
        3,
        objdetect::CASCADE_SCALE_IMAGE,
        core::Size::new(1, 1),
        core::Size::new(10000000, 10000000),
    )?;
    Ok(faces)
}

// 简单特征匹配示例
fn is_face_matching(face: &Rect, known_faces: &Vec<Rect>) -> bool {
    for known_face in known_faces {
        if (face.x == known_face.x)
            && (face.y == known_face.y)
            && (face.width == known_face.width)
            && (face.height == known_face.height)
        {
            return true;
        }
    }
    false
}

fn main() -> Result<()> {
    // 加载人脸检测模型
    let mut face_cascade =
        objdetect::CascadeClassifier::new("haarcascade_frontalface_default.xml")?;

    // 记录人脸的特征
    let mut known_faces = Vec::new();

    // 读取记录人脸照片
    let image = imgcodecs::imread("20241023092653.jpg", imgcodecs::IMREAD_COLOR)?;
    println!("读取记录人脸照片mat： {:?}", image.size());
    let faces = detect_faces(&mut face_cascade, &image)?;
    println!("读取记录人脸照片vec： {:?}", faces);

    for face in faces {
        known_faces.push(face);
    }

    // 初始化摄像头
    let mut cam = VideoCapture::new(0, CAP_ANY)?;
    if !cam.is_opened()? {
        panic!("摄像头未打开！");
    }

    loop {
        let mut frame = Mat::default();
        cam.read(&mut frame)?;
        println!("摄像头mat： {:?}", frame.size());

        // 检测当前画面中的人脸
        let faces = detect_faces(&mut face_cascade, &frame)?;

        // 绘制矩形
        for face in faces {
            imgproc::rectangle(
                &mut frame,
                face,
                core::Scalar::new(0.0, 255.0, 0.0, 0.0),
                1,
                8,
                0,
            )?;
            // 保存人脸图片
            let face_image = Mat::roi(&frame, face)?;
            imgcodecs::imwrite("saved_face.jpg", &face_image, &core::Vector::new())?;
            println!("保存人脸图片");

            // 检查当前人脸是否匹配已知人脸
            println!("当前人脸：{:?}", face);
            println!("已知人脸：{:?}", known_faces);
            if is_face_matching(&face, &known_faces) {
                println!("匹配成功！");
            }
        }

        // 显示结果
        highgui::imshow("人脸识别", &frame)?;
        if highgui::wait_key(10)? > 0 {
            break;
        }
    }

    Ok(())
}

// 说明
// 人脸检测：使用 detect_faces 函数，该函数将图像转换为灰度并使用 Haar 级联分类器检测人脸。
// 特征匹配：这个示例中包含了一个简单的 is_face_matching 函数，通过比较矩形区域的位置和大小来判断人脸是否匹配。实际的人脸识别通常需要使用更多的特征提取算法。
// 流媒体识别：程序持续从摄像头流中读取帧，进行人脸检测，并在检测到的人脸上绘制矩形。
// 注意事项
// 确保 OpenCV 已正确安装，并且 haarcascade_frontalface_default.xml 文件可访问。
// 根据需求，真正的特征匹配可以使用更复杂的算法，如 Eigenfaces、Fisherfaces 或 LBPH，来获取更精确的人脸匹配和识别。
// 该代码仅为基本框架，实际应用时需要考虑更多的错误处理和优化。
// 这样，你就有了一个基本的人脸检测和识别功能，可以识别从图片中记录的脸并通过摄像头流进行实时检测。你可以根据自己的需求扩展功能和细节。
