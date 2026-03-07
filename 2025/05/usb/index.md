# usb笔记


<!--more-->

**USB学习笔记总结**

<!--more-->

## usb基础

### usb class

> USB-IF通过**设备类代码**（bInterfaceClass）标准化设备类型，便于操作系统自动识别驱动：

1. **人机交互设备（HID, Class 0x03）**
   - **设备**：键盘、鼠标、游戏手柄、触摸屏。
   - **特点**：无需额外驱动（操作系统内置支持）。

2. **大容量存储设备（MSC, Class 0x08）**
   - **设备**：U盘、移动硬盘、SD读卡器。
   - **协议**：采用BOT（Bulk-Only Transport）或UASP（USB Attached SCSI Protocol）协议，后者速度更快且支持多队列。

3. **音视频设备**
   - **音频类（ADC, Class 0x01）**：麦克风、音箱、USB声卡。
   - **视频类（UVC, Class 0x0E）**：网络摄像头、视频采集卡（支持免驱即插即用）。

4. **通信设备（CDC, Class 0x02）**
   - **设备**：4G模块、USB网卡、调制解调器。
   - **扩展**：含虚拟串口（如CH340芯片）。

5. **图像设备（IDC, Class 0x06）**
   - **设备**：扫描仪、数码相机（部分型号通过MSC传输照片）。

6. **其他专业设备类**
   - **打印设备（Class 0x07）**：喷墨/激光打印机。
   - **智能卡设备（Class 0x0B）**：银行U盾、门禁读卡器。
   - **医疗健康设备（Class 0x0F）**：血糖仪、心率监测器。

## tinyusb开发
TBD

## libusb开发
TBD

## linux/usb开发
TBD
