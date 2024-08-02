import { useEffect, useState } from "react";
import { useSelector } from "react-redux";
import { ConfigProvider, Drawer, message, theme } from "antd";
import zhCN from "antd/locale/zh_CN";
import { invoke } from "@tauri-apps/api";
import MainForm, { FormState } from "./components/MainForm";
import SettingsForm from "./components/SettingsForm";
import FigureZone, { PowerRecords, WorkRecords } from "./components/FigureZone";
import { RootState } from "./store";
import "./App.css";

interface Response {
    p: PowerRecords;
    w: WorkRecords;
}

export default function App() {
    // 侧边栏是否打开
    const [open, setOpen] = useState(false);
    // 图表是否为加载中
    const [spinning, setSpinning] = useState(false);

    const emptyPowerRecords: PowerRecords = {
        eo: { x: [] as string[], y: [] as number[] },
        mp: { x: [] as string[], y: [] as number[] },
        no: { x: [] as string[], y: [] as number[] },
        np: { x: [] as string[], y: [] as number[] },
        er: { x: [] as string[], y: [] as number[] },
        nr: { x: [] as string[], y: [] as number[] },
    };
    const emptyWorkRecords: WorkRecords = {
        m: { x: [] as string[], y: [] as number[] },
        n: { x: [] as string[], y: [] as number[] },
    };

    // 后端处理返回的全部功率、能耗数据
    const [powerRecords, setPowerRecords] = useState<PowerRecords>(emptyPowerRecords);
    const [workRecords, setWorkRecords] = useState<WorkRecords>(emptyWorkRecords);

    const requestFigureData = async (state: FormState) => {
        setPowerRecords(emptyPowerRecords);
        setWorkRecords(emptyWorkRecords);
        setSpinning(true);

        try {
            // @ts-ignore
            const resp: Response = await invoke("build_series", state);

            setPowerRecords(resp.p);
            setWorkRecords(resp.w);
        } catch (e) {
            const msg = `数据处理发生错误：${e as string}`;
            console.error(msg);
            message.error(msg);
        }

        setSpinning(false);
    };

    const darkMode = useSelector<RootState>((state) => state.settings.darkMode);
    useEffect(() => {
        if (darkMode) {
            document.body.style.backgroundColor = "#333";
        } else {
            document.body.style.backgroundColor = "#FFF";
        }
    }, [darkMode]);

    return (
        <div className="container">
            <ConfigProvider locale={zhCN} theme={{ algorithm: darkMode ? theme.darkAlgorithm : theme.defaultAlgorithm }}>

                <MainForm
                    onSubmit={requestFigureData}
                    onRequestOpenDrawer={() => setOpen(true)}
                />

                <FigureZone
                    powerRecords={powerRecords}
                    workRecords={workRecords}
                    spinning={spinning}
                />

                <Drawer
                    open={open}
                    onClose={() => { setOpen(false); }}
                    placement='left'
                >
                    <SettingsForm />
                </Drawer>

            </ConfigProvider>
        </div>
    )
}
