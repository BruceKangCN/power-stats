import { useState } from "react";
import { Form, Select } from "antd";
import { store } from "../store";
import { setMonth } from "./SettingsSlice";

export interface FieldData {
    name: string | number | (string | number)[];
    value?: any;
    touched?: boolean;
    validating?: boolean;
    errors?: string[];
}

const SettingsForm: React.FC = () => {
    const [fields, _] = useState<FieldData[]>([
        { name: "month", value: "all" },
    ]);

    return (
        <Form fields={fields}>

            <Form.Item
                label="月份"
                name="month"
            >
                <Select
                    options={[
                        { value: "all", label: "全部" },
                        { value: "01", label: "1" },
                        { value: "02", label: "2" },
                        { value: "03", label: "3" },
                        { value: "04", label: "4" },
                        { value: "05", label: "5" },
                        { value: "06", label: "6" },
                        { value: "07", label: "7" },
                        { value: "08", label: "8" },
                        { value: "09", label: "9" },
                        { value: "10", label: "10" },
                        { value: "11", label: "11" },
                        { value: "12", label: "12" },
                    ]}
                    onChange={(value) => { store.dispatch(setMonth(value)); }}
                />
            </Form.Item>

        </Form>
    );
};

export default SettingsForm;
