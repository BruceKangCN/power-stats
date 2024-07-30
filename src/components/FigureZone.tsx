import { useSelector } from 'react-redux';
import { useMemo } from 'react';
import { Spin, Tabs } from "antd";
import Plot from "react-plotly.js";
import { Config, Layout, PlotData } from "plotly.js";
// @ts-ignore
import zh_CN from "plotly.js/lib/locales/zh-cn";
import { RootState } from '../store';
import "./FigureZone.css";

interface Series {
    x: string[];
    y: number[];
}

export interface PowerRecords {
    eo: Series; // 晚谷瞬时功耗
    mp: Series; // 早峰瞬时功耗
    no: Series; // 午谷瞬时功耗
    np: Series; // 午峰瞬时功耗
    er: Series; // 晚谷余量
    nr: Series; // 午谷余量
}

export interface WorkRecords {
    m: Series; // 早峰能耗
    n: Series; // 午峰能耗
}

interface FigureZoneProps {
    powerRecords: PowerRecords;
    workRecords: WorkRecords;
    spinning: boolean;
}

const FigureZone: React.FC<FigureZoneProps> = ({ powerRecords, workRecords, spinning }) => {
    const layout: Partial<Layout> = {
        autosize: true,
        margin: {
            b: 30,
            l: 30,
            r: 30,
            t: 30,
        },
        hovermode: "x",
    };

    const month = useSelector<RootState>((state) => state.settings.month);

    const config: Partial<Config> = {
        locales: { "zh-CN": zh_CN },
        locale: "zh-CN",
        displayModeBar: true,
        displaylogo: false,
        responsive: true,
        scrollZoom: true,
    }

    const eoSeries = useMemo<Series>(() => {
        const eo = powerRecords.eo;

        if (month === "all") {
            return eo;
        }

        const s: Series = {
            x: [],
            y: [],
        };
        for (let i = 0; i < eo.x.length; i++) {
            if (eo.x[i].slice(5, 7) === month) {
                s.x.push(eo.x[i]);
                s.y.push(eo.y[i]);
            }
        }

        return s;
    }, [powerRecords, month]);

    const mpSeries = useMemo<Series>(() => {
        const mp = powerRecords.mp;

        if (month === "all") {
            return mp;
        }

        const s: Series = {
            x: [],
            y: [],
        };
        for (let i = 0; i < mp.x.length; i++) {
            if (mp.x[i].slice(5, 7) === month) {
                s.x.push(mp.x[i]);
                s.y.push(mp.y[i]);
            }
        }

        return s;
    }, [powerRecords, month]);

    const noSeries = useMemo<Series>(() => {
        const no = powerRecords.no;

        if (month === "all") {
            return no;
        }

        const s: Series = {
            x: [],
            y: [],
        };
        for (let i = 0; i < no.x.length; i++) {
            if (no.x[i].slice(5, 7) === month) {
                s.x.push(no.x[i]);
                s.y.push(no.y[i]);
            }
        }

        return s;
    }, [powerRecords, month]);

    const npSeries = useMemo<Series>(() => {
        const np = powerRecords.np;

        if (month === "all") {
            return np;
        }

        const s: Series = {
            x: [],
            y: [],
        };
        for (let i = 0; i < np.x.length; i++) {
            if (np.x[i].slice(5, 7) === month) {
                s.x.push(np.x[i]);
                s.y.push(np.y[i]);
            }
        }

        return s;
    }, [powerRecords, month]);

    const erSeries = useMemo<Series>(() => {
        const er = powerRecords.er;

        if (month === "all") {
            return er;
        }

        const s: Series = {
            x: [],
            y: [],
        };
        for (let i = 0; i < er.x.length; i++) {
            if (er.x[i].slice(5, 7) === month) {
                s.x.push(er.x[i]);
                s.y.push(er.y[i]);
            }
        }

        return s;
    }, [powerRecords, month]);

    const nrSeries = useMemo<Series>(() => {
        const nr = powerRecords.nr;

        if (month === "all") {
            return nr;
        }

        const s: Series = {
            x: [],
            y: [],
        };
        for (let i = 0; i < nr.x.length; i++) {
            if (nr.x[i].slice(5, 7) === month) {
                s.x.push(nr.x[i]);
                s.y.push(nr.y[i]);
            }
        }

        return s;
    }, [powerRecords, month]);

    const powerData: Partial<PlotData>[] = useMemo(() => {
        return [
            {
                name: "晚谷瞬时功率",
                x: eoSeries.x,
                y: eoSeries.y,
                type: "scatter",
                mode: "lines+markers",
            },
            {
                name: "早峰瞬时功率",
                x: mpSeries.x,
                y: mpSeries.y,
                type: "scatter",
                mode: "lines+markers",
            },
            {
                name: "午谷瞬时功率",
                x: noSeries.x,
                y: noSeries.y,
                type: "scatter",
                mode: "lines+markers",
            },
            {
                name: "午峰瞬时功率",
                x: npSeries.x,
                y: npSeries.y,
                type: "scatter",
                mode: "lines+markers",
            },
            {
                name: "晚谷余量",
                x: erSeries.x,
                y: erSeries.y,
                type: "scatter",
                mode: "lines+markers",
            },
            {
                name: "午谷余量",
                x: nrSeries.x,
                y: nrSeries.y,
                type: "scatter",
                mode: "lines+markers",
            },
        ];
    }, [eoSeries, mpSeries, noSeries, npSeries, erSeries, nrSeries, month]);

    const mSeries = useMemo<Series>(() => {
        const m = workRecords.m;

        if (month === "all") {
            return m;
        }

        const s: Series = {
            x: [],
            y: [],
        };
        for (let i = 0; i < m.x.length; i++) {
            if (m.x[i].slice(5, 7) === month) {
                s.x.push(m.x[i]);
                s.y.push(m.y[i]);
            }
        }

        return s;
    }, [powerRecords, month]);

    const nSeries = useMemo<Series>(() => {
        const n = workRecords.n;

        if (month === "all") {
            return n;
        }

        const s: Series = {
            x: [],
            y: [],
        };
        for (let i = 0; i < n.x.length; i++) {
            if (n.x[i].slice(5, 7) === month) {
                s.x.push(n.x[i]);
                s.y.push(n.y[i]);
            }
        }

        return s;
    }, [powerRecords, month]);

    const workData: Partial<PlotData>[] = useMemo(() => {
        return [
            {
                name: "早峰能耗",
                x: mSeries.x,
                y: mSeries.y,
                type: "scatter",
                mode: "lines+markers",
            },
            {
                name: "午峰能耗",
                x: nSeries.x,
                y: nSeries.y,
                type: "scatter",
                mode: "lines+markers",
            },
        ];
    }, [workRecords, month]);

    const powerFigure = (
        <Spin spinning={spinning}>
            <Plot
                data={powerData}
                layout={{...layout, xaxis: { tickformat: "%Y-%m-%d %H:%M:%S" }} as any}
                config={config as any}
                className="chart"
            />
        </Spin>
    );

    const workFigure = (
        <Spin spinning={spinning}>
            <Plot
                data={workData}
                layout={{...layout, xaxis: { tickformat: "%Y-%m-%d" }} as any}
                config={config as any}
                className="chart"
            />
        </Spin>
    );

    return (
        <Tabs
            centered
            defaultActiveKey="1"
            items={[
                { key: "1", label: "功率图", children: powerFigure },
                { key: "2", label: "能耗图", children: workFigure },
            ]}
        />
    );
};

export default FigureZone;
