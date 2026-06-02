use leptos::prelude::*;
use leptos_meta::Script;
use crate::app::{get_streamer_analytics, StreamerAnalytics};

// ─── JS Interop ──────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "hydrate")]
#[wasm_bindgen(inline_js = r#"
export function init_echarts_line(dom_id, x_data, y_data, series_name, color) {
    let dom = document.getElementById(dom_id);
    if (!dom) return;
    let chart = echarts.init(dom, 'dark', { renderer: 'canvas' });
    chart.setOption({
        backgroundColor: 'transparent',
        tooltip: { trigger: 'axis', formatter: function(p) { return p[0].name + '<br/>' + p[0].seriesName + ': $' + p[0].value.toFixed(2); } },
        grid: { left: '3%', right: '4%', bottom: '3%', containLabel: true },
        xAxis: { type: 'category', data: x_data, axisLine: { lineStyle: { color: '#ffffff30' } }, axisLabel: { color: '#aaa', fontSize: 11 } },
        yAxis: { type: 'value', axisLabel: { color: '#aaa', formatter: '${value}' }, splitLine: { lineStyle: { color: '#ffffff10' } } },
        series: [{ name: series_name, type: 'line', smooth: true, data: y_data,
            lineStyle: { color: color, width: 3 },
            itemStyle: { color: color },
            areaStyle: { color: { type: 'linear', x: 0, y: 0, x2: 0, y2: 1,
                colorStops: [{ offset: 0, color: color + '60' }, { offset: 1, color: color + '00' }] } }
        }]
    });
    window.addEventListener('resize', () => chart.resize());
}

export function init_echarts_hbar(dom_id, names, values, color) {
    let dom = document.getElementById(dom_id);
    if (!dom) return;
    let chart = echarts.init(dom, 'dark', { renderer: 'canvas' });
    chart.setOption({
        backgroundColor: 'transparent',
        tooltip: { trigger: 'axis', formatter: function(p) { return p[0].name + ': $' + p[0].value.toFixed(2); } },
        grid: { left: '3%', right: '8%', bottom: '3%', top: '3%', containLabel: true },
        xAxis: { type: 'value', axisLabel: { color: '#aaa', formatter: '${value}' }, splitLine: { lineStyle: { color: '#ffffff10' } } },
        yAxis: { type: 'category', data: names, axisLabel: { color: '#ccc', fontSize: 12 } },
        series: [{ type: 'bar', data: values, barMaxWidth: 28,
            itemStyle: { color: color, borderRadius: [0, 6, 6, 0] },
            label: { show: true, position: 'right', formatter: function(p) { return '$' + p.value.toFixed(2); }, color: '#ccc', fontSize: 11 }
        }]
    });
    window.addEventListener('resize', () => chart.resize());
}

export function init_echarts_pie(dom_id, data_json) {
    let dom = document.getElementById(dom_id);
    if (!dom) return;
    let chart = echarts.init(dom, 'dark', { renderer: 'canvas' });
    let data = JSON.parse(data_json);
    let colors = ['#6750A4', '#B58392', '#7C9A92', '#E8A838', '#4FC3F7', '#81C784'];
    chart.setOption({
        backgroundColor: 'transparent',
        tooltip: { trigger: 'item', formatter: '{b}: {c} donations ({d}%)' },
        legend: { orient: 'vertical', right: 10, top: 'center', textStyle: { color: '#ccc' } },
        series: [{
            type: 'pie', radius: ['45%', '75%'], center: ['38%', '50%'],
            avoidLabelOverlap: false,
            label: { show: false },
            emphasis: { label: { show: true, fontSize: 14, fontWeight: 'bold' } },
            data: data.map(function(d, i) { return { name: d[0], value: d[1], itemStyle: { color: colors[i % colors.length] } }; })
        }]
    });
    window.addEventListener('resize', () => chart.resize());
}

export function init_echarts_vbar(dom_id, x_data, y_data, color) {
    let dom = document.getElementById(dom_id);
    if (!dom) return;
    let chart = echarts.init(dom, 'dark', { renderer: 'canvas' });
    chart.setOption({
        backgroundColor: 'transparent',
        tooltip: { trigger: 'axis', formatter: function(p) { return p[0].name + ': ' + p[0].value + ' donations'; } },
        grid: { left: '3%', right: '4%', bottom: '3%', containLabel: true },
        xAxis: { type: 'category', data: x_data, axisLabel: { color: '#aaa', fontSize: 11 } },
        yAxis: { type: 'value', axisLabel: { color: '#aaa' }, splitLine: { lineStyle: { color: '#ffffff10' } }, minInterval: 1 },
        series: [{ type: 'bar', data: y_data, barMaxWidth: 52,
            itemStyle: { color: color, borderRadius: [6, 6, 0, 0] },
            label: { show: true, position: 'top', color: '#ccc', fontSize: 12 }
        }]
    });
    window.addEventListener('resize', () => chart.resize());
}
"#)]
#[cfg(feature = "hydrate")]
extern "C" {
    pub fn init_echarts_line(
        dom_id: &str,
        x_data: js_sys::Array,
        y_data: js_sys::Array,
        series_name: &str,
        color: &str,
    );
    pub fn init_echarts_hbar(
        dom_id: &str,
        names: js_sys::Array,
        values: js_sys::Array,
        color: &str,
    );
    pub fn init_echarts_pie(dom_id: &str, data_json: &str);
    pub fn init_echarts_vbar(
        dom_id: &str,
        x_data: js_sys::Array,
        y_data: js_sys::Array,
        color: &str,
    );
}

// ─── KPI Card ─────────────────────────────────────────────────────────────────

#[component]
fn KpiCard(
    icon: &'static str,
    label: &'static str,
    value: String,
    color: &'static str,
) -> impl IntoView {
    let color_class = match color {
        "primary" => "text-primary",
        "secondary" => "text-secondary",
        "tertiary" => "text-tertiary",
        _ => "text-on-surface",
    };
    let icon_bg = match color {
        "primary" => "bg-primary/15",
        "secondary" => "bg-secondary/15",
        "tertiary" => "bg-tertiary/15",
        _ => "bg-surface-variant/40",
    };
    view! {
        <div class="bg-surface-container-low/40 backdrop-blur-md border border-white/10 rounded-2xl p-lg flex flex-col gap-sm">
            <div class="flex items-center gap-sm">
                <div class=format!(
                    "w-10 h-10 rounded-xl flex items-center justify-center {}",
                    icon_bg,
                )>
                    <span class=format!("material-symbols-outlined {}", color_class)>{icon}</span>
                </div>
                <span class="text-label-md text-on-surface-variant font-medium">{label}</span>
            </div>
            <p class=format!("text-display-sm font-bold font-mono {}", color_class)>{value}</p>
        </div>
    }
}

// ─── Chart container ─────────────────────────────────────────────────────────

#[component]
fn ChartCard(
    icon: &'static str,
    title: String,
    chart_id: &'static str,
    #[prop(optional)] height: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let height = height.unwrap_or("h-72");
    view! {
        <div class="bg-surface-container-low/40 backdrop-blur-md border border-white/10 rounded-2xl p-lg flex flex-col gap-md">
            <h3 class="text-title-lg font-bold text-on-surface flex items-center gap-sm">
                <span class="material-symbols-outlined text-primary">{icon}</span>
                {title}
            </h3>
            <div id=chart_id class=format!("w-full {}", height)></div>
            {children()}
        </div>
    }
}

// ─── Main Analytics Page ───────────────────────────────────────────────────────

#[component]
pub fn AnalyticsPage() -> impl IntoView {
    let streamer = use_context::<crate::db::DbStreamer>().expect("Streamer context missing");
    let (time_range, set_time_range) = signal("week".to_string());
    // ← The key fix: track when ECharts CDN has finished loading
    let (echarts_ready, set_echarts_ready) = signal(false);

    let analytics_resource = LocalResource::new(move || {
        let streamer_id = streamer.id;
        let range = time_range.get();
        async move {
            crate::utils::with_min_delay(get_streamer_analytics(streamer_id, range)).await
        }
    });

    view! {
        // Script fires on:load → sets echarts_ready = true
        <Script
            src="https://cdn.jsdelivr.net/npm/echarts@5/dist/echarts.min.js"
            on:load=move |_| set_echarts_ready.set(true)
        />

        <div class="flex flex-col gap-lg mb-xl">
            // Header + time range toggle
            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-md">
                <h1 class="text-headline-lg font-headline-lg text-on-surface">
                    {leptos_fluent::move_tr!("analytics-title")}
                </h1>
                <div class="flex bg-surface-variant/50 p-1 rounded-xl self-start">
                    {["day", "week", "month"]
                        .into_iter()
                        .map(|range| {
                            let label = match range {
                                "day" => "24h",
                                "week" => "7d",
                                "month" => "30d",
                                _ => "",
                            };
                            view! {
                                <button
                                    class=move || {
                                        format!(
                                            "px-4 py-1.5 rounded-lg text-label-sm font-bold transition-all {}",
                                            if time_range.get() == range {
                                                "bg-primary text-on-primary shadow-md"
                                            } else {
                                                "text-on-surface-variant hover:text-on-surface"
                                            },
                                        )
                                    }
                                    on:click=move |_| set_time_range.set(range.to_string())
                                >
                                    {label}
                                </button>
                            }
                        })
                        .collect_view()}
                </div>
            </div>

            <Suspense fallback=move || {
                view! {
                    <div class="grid grid-cols-2 md:grid-cols-4 gap-md animate-pulse">
                        {(0..4)
                            .map(|_| {
                                view! {
                                    <div class="h-28 bg-surface-container-low/40 rounded-2xl" />
                                }
                            })
                            .collect_view()}
                    </div>
                }
            }>
                {move || {
                    analytics_resource
                        .get()
                        .map(|res| match res {
                            Err(e) => {
                                view! {
                                    <div class="text-error bg-error/10 p-md rounded-xl">
                                        {format!("Failed to load analytics: {:?}", e)}
                                    </div>
                                }
                                    .into_any()
                            }
                            Ok(analytics) => {
                                view! {
                                    // Only render charts after ECharts script is loaded
                                    <Show
                                        when=move || echarts_ready.get()
                                        fallback=move || {
                                            view! {
                                                <div class="grid grid-cols-2 md:grid-cols-4 gap-md animate-pulse">
                                                    {(0..4)
                                                        .map(|_| {
                                                            view! {
                                                                <div class="h-28 bg-surface-container-low/40 rounded-2xl" />
                                                            }
                                                        })
                                                        .collect_view()}
                                                </div>
                                            }
                                        }
                                    >
                                        <AnalyticsDashboard analytics=analytics.clone() />
                                    </Show>
                                }
                                    .into_any()
                            }
                        })
                }}
            </Suspense>
        </div>
    }
}

// ─── Dashboard assembled from charts ─────────────────────────────────────────

#[component]
fn AnalyticsDashboard(analytics: StreamerAnalytics) -> impl IntoView {
    // Serialize data for chart init (called after mount via Effect)
    #[cfg(feature = "hydrate")]
    {
        use leptos::prelude::Effect;
        let rev_dates: Vec<String> = analytics.revenue_over_time.iter().map(|(d, _)| d.clone()).collect();
        let rev_vals: Vec<f64> = analytics.revenue_over_time.iter().map(|(_, v)| *v).collect();
        let cumul_dates: Vec<String> = analytics.cumulative_revenue.iter().map(|(d, _)| d.clone()).collect();
        let cumul_vals: Vec<f64> = analytics.cumulative_revenue.iter().map(|(_, v)| *v).collect();
        let donor_names: Vec<String> = analytics.top_donors.iter().rev().map(|(n, _)| n.clone()).collect();
        let donor_vals: Vec<f64> = analytics.top_donors.iter().rev().map(|(_, v)| *v).collect();
        let dist_labels: Vec<String> = analytics.amount_distribution.iter().map(|(l, _)| l.clone()).collect();
        let dist_vals: Vec<i64> = analytics.amount_distribution.iter().map(|(_, v)| *v).collect();
        let pie_json = {
            let parts: Vec<String> = analytics.payment_method_breakdown.iter()
                .map(|(name, cnt)| format!("[\"{}\",{}]", name.replace('"', "\\\""), cnt))
                .collect();
            format!("[{}]", parts.join(","))
        };

        Effect::new(move |_| {
            // Line chart: Revenue over time
            let x: js_sys::Array = rev_dates.iter().map(|s| wasm_bindgen::JsValue::from_str(s)).collect();
            let y: js_sys::Array = rev_vals.iter().map(|v| wasm_bindgen::JsValue::from_f64(*v)).collect();
            init_echarts_line("chart-revenue-time", x, y, "Revenue", "#6750A4");

            // Area chart: Cumulative revenue
            let cx: js_sys::Array = cumul_dates.iter().map(|s| wasm_bindgen::JsValue::from_str(s)).collect();
            let cy: js_sys::Array = cumul_vals.iter().map(|v| wasm_bindgen::JsValue::from_f64(*v)).collect();
            init_echarts_line("chart-cumulative", cx, cy, "Cumulative", "#4FC3F7");

            // Horizontal bar: Top donors
            let dn: js_sys::Array = donor_names.iter().map(|s| wasm_bindgen::JsValue::from_str(s)).collect();
            let dv: js_sys::Array = donor_vals.iter().map(|v| wasm_bindgen::JsValue::from_f64(*v)).collect();
            init_echarts_hbar("chart-top-donors", dn, dv, "#B58392");

            // Pie: Payment method
            init_echarts_pie("chart-payment-method", &pie_json);

            // Vertical bar: Amount distribution
            let dl: js_sys::Array = dist_labels.iter().map(|s| wasm_bindgen::JsValue::from_str(s)).collect();
            let dv2: js_sys::Array = dist_vals.iter().map(|v| wasm_bindgen::JsValue::from_f64(*v as f64)).collect();
            init_echarts_vbar("chart-amount-dist", dl, dv2, "#7C9A92");
        });
    }

    view! {
        // KPI Summary Cards
        <div class="grid grid-cols-2 md:grid-cols-4 gap-md">
            <KpiCard
                icon="account_balance_wallet"
                label="Total Revenue"
                value=format!("${:.2}", analytics.total_revenue)
                color="primary"
            />
            <KpiCard
                icon="favorite"
                label="Total Donations"
                value=analytics.donation_count.to_string()
                color="secondary"
            />
            <KpiCard
                icon="payments"
                label="Avg. Donation"
                value=format!("${:.2}", analytics.avg_donation)
                color="tertiary"
            />
            <KpiCard
                icon="star"
                label="Largest Single"
                value=format!("${:.2}", analytics.top_single_donation)
                color="primary"
            />
        </div>

        // Row 1: Revenue Over Time + Cumulative
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-md">
            <ChartCard
                icon="show_chart"
                title="Revenue Over Time".to_string()
                chart_id="chart-revenue-time"
                height="h-72"
            >
                <span />
            </ChartCard>
            <ChartCard
                icon="trending_up"
                title="Cumulative Revenue".to_string()
                chart_id="chart-cumulative"
                height="h-72"
            >
                <span />
            </ChartCard>
        </div>

        // Row 2: Top Donors + Payment Method
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-md">
            <ChartCard
                icon="emoji_events"
                title="Top Donors".to_string()
                chart_id="chart-top-donors"
                height="h-80"
            >
                <span />
            </ChartCard>
            <ChartCard
                icon="pie_chart"
                title="Payment Method Breakdown".to_string()
                chart_id="chart-payment-method"
                height="h-80"
            >
                <span />
            </ChartCard>
        </div>

        // Row 3: Amount Distribution full width
        <ChartCard
            icon="bar_chart"
            title="Donation Amount Distribution".to_string()
            chart_id="chart-amount-dist"
            height="h-64"
        >
            <span />
        </ChartCard>
    }
}
