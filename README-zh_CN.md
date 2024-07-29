# 电力统计

电力统计 App，可根据电力数据绘制功率、能耗图，并根据用户输入的变压器额定容量、负载倍率计算谷时余量，以便于判断是否适合投资储能项目。

## 技术框架

使用 Tauri 应用框架搭建，前端使用 TypeScript 语言，后端使用 Rust 语言。

前端：
- 框架：Vue 3
- UI：Ant Design Vue
- 图表：Apache ECharts

后端：
- 框架：Tauri
- 文本编码识别：charset-normalizer
