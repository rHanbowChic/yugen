from PIL import Image
import os

def resize_app_icon():
    input_filename = "app-icon-any-size.png"
    output_filename = "app-icon.png"
    target_size = (1024, 1024)

    # 检查文件是否存在
    if not os.path.exists(input_filename):
        print(f"错误: 在当前路径下找不到 {input_filename}")
        return

    try:
        # 打开图片
        with Image.open(input_filename) as img:
            # 确保图片包含透明度通道 (RGBA)
            img = img.convert("RGBA")

            # 使用线性插值 (BILINEAR) 进行缩放
            # 如果你追求更高质量，也可以使用 Image.Resampling.LANCZOS
            resized_img = img.resize(target_size, resample=Image.Resampling.BILINEAR)

            # 保存结果，保留透明度
            resized_img.save(output_filename, "PNG")

            print(f"成功！调整后的图标已保存为: {output_filename}")

    except Exception as e:
        print(f"处理过程中出现错误: {e}")

if __name__ == "__main__":
    resize_app_icon()
