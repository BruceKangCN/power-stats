import { useState } from "react";
import { Button, Form, Input, InputNumber, Space, Switch } from "antd";
import { open } from "@tauri-apps/api/dialog";

export interface FormState {
  ratedCapacity: number; // 额定容量
  isPrimaryLoad: boolean; // 是否为一次负荷
  factor?: number; // 倍率
  filepath: string; // 数据文件路径
}

export interface FormProps {
    onSubmit: (formState: FormState) => void;
    onRequestOpenDrawer: () => void;
}

const MainForm: React.FC<FormProps> = ({onSubmit, onRequestOpenDrawer}) => {
    const initialValues: FormState = {
        ratedCapacity: 0.0,
        isPrimaryLoad: true,
        factor: 1.0,
        filepath: "",
    };

    const [filepath, setFilepath] = useState("");

    const [form] = Form.useForm();
    const isPrimaryLoad = Form.useWatch("isPrimaryLoad", form);

    const openFile = async () => {
        const p = await open();
        if (p) {
            if (typeof(p) === "string") {
                setFilepath(p);
                form.setFieldValue("filepath", p);
            } else {
                setFilepath(p[0]);
                form.setFieldValue("filepath", p[0]);
            }
        }
    };

    return (
        <Form
            form={form}
            name="formState"
            labelCol={{ span: 6 }}
            wrapperCol={{ span: 16 }}
            autoComplete="off"
            initialValues={initialValues}
            onFinish={onSubmit}
        >

            <Form.Item
                label="变压器额定容量"
                name="ratedCapacity"
                rules={[{ required: true, message: "请输入额定容量" }]}
            >
                <InputNumber />
            </Form.Item>

            <Form.Item
                label="是否为一次负荷"
                name="isPrimaryLoad"
                rules={[{ required: true, message: "请选择是否为一次负荷" }]}
                valuePropName="checked"
            >
                <Switch />
            </Form.Item>

            <Form.Item
                label="倍率"
                name="factor"
                rules={[{ required: isPrimaryLoad, message: "一次负载需填写倍率" }]}
            >
                <InputNumber />
            </Form.Item>

            <Form.Item
                label="数据源"
                name="filepath"
                rules={[{ required: true, message: "请选择数据文件" }]}
            >
                <Space>
                    <Input value={filepath} onChange={(e) => { setFilepath(e.target.value); }} />
                    <Button onClick={openFile}>...</Button>
                </Space>
            </Form.Item>

            <Form.Item wrapperCol={{ offset: 6, span: 16 }}>
                <Button
                    onClick={onRequestOpenDrawer}
                    style={{ marginRight: 10 }}
                >选项</Button>
                <Button type="primary" htmlType="submit">确认</Button>
            </Form.Item>

        </Form>
    );
};

export default MainForm;
