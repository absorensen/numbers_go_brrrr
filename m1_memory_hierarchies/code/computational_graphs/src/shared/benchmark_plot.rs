use plotters::prelude::*;

use super::performance_measurement::PerformanceMeasurements;

// Function based on https://plotters-rs.github.io/book/basic/basic_data_plotting.html
pub fn draw_benchmark_plot(
    chart_name: &str,
    path: &str,
    file_name: &str,
    measurements: Vec<PerformanceMeasurements>,
    log_scale: bool,
) {
    let plot_resolution: (u32, u32) = (1200, 800);
    let x_label_area_size: i32 = 100;
    let y_label_area_size: i32 = 100;
    let right_y_label_area_size: i32 = 200;
    let margin: i32 = 25;
    let title_font_size: i32 = 25;

    let x_label: &str = "Element Count";
    let y_label: &str = "Nanseconds";

    //
    // No tweaking beyond this point!
    //

    // Setup
    let mut output_name: String = "outputs/".to_string();
    output_name.push_str(path);

    // If directories do not exist - create them
    use std::fs;
    fs::create_dir_all(&output_name)
        .expect("Failed to create necessary directories for plot outputs.");

    output_name.push_str(file_name);

    // Get the minimum and maximum values on each axis for all measurements
    let mut min_value_x_axis: i32 = i32::MAX;
    let mut max_value_x_axis: i32 = i32::MIN;
    let mut min_value_y_axis: f32 = f32::MAX;
    let mut max_value_y_axis: f32 = f32::MIN;
    for measurement in &measurements {
        let min_value_x: i32 = *measurement
            .sizes
            .iter()
            .min_by_key(|x| *(*x))
            .expect("Unable to find the min value of the x axis in draw_performance_plot.")
            as i32;
        let max_value_x: i32 = *measurement
            .sizes
            .iter()
            .max_by_key(|x| *(*x))
            .expect("Unable to find the max value of the x axis in draw_performance_plot.")
            as i32;

        min_value_x_axis = min_value_x_axis.min(min_value_x);
        max_value_x_axis = max_value_x_axis.max(max_value_x);

        use ordered_float::OrderedFloat;
        let min_value_y: f32 = *measurement
            .normalized_times
            .iter()
            .min_by_key(|x| OrderedFloat(x.abs()))
            .expect("Unable to find the min value of the y axis in draw_performance_plot.");
        let max_value_y: f32 = *measurement
            .normalized_times
            .iter()
            .max_by_key(|x| OrderedFloat(x.abs()))
            .expect("Unable to find the max value of the y axis in draw_performance_plot.");

        min_value_y_axis = min_value_y_axis.min(min_value_y);
        max_value_y_axis = max_value_y_axis.max(max_value_y);
    }

    // Draw
    let root_area = BitMapBackend::new(output_name.as_str(), plot_resolution).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    // The reason for this really unnecessary looking if statement
    // is that using .log_scale() in .build_cartesian_2d() results in chart
    // resolving to a different type.
    // There are probably ways to minimize the divergence between
    // the two paths, but this is a beginner friendly tutorial
    if log_scale {
        let mut chart = ChartBuilder::on(&root_area)
            .x_label_area_size(x_label_area_size)
            .y_label_area_size(y_label_area_size)
            .right_y_label_area_size(right_y_label_area_size)
            .margin(margin)
            .caption(chart_name, ("sans-serif", title_font_size))
            .build_cartesian_2d(
                (min_value_x_axis..max_value_x_axis).log_scale(),
                (min_value_y_axis..max_value_y_axis).log_scale(),
            )
            .unwrap();

        chart
            .configure_mesh()
            .x_desc(x_label)
            .y_desc(y_label)
            .draw()
            .unwrap();

        for (measurement_index, measurement) in measurements.iter().enumerate() {
            let zipped_data: Vec<(usize, f32)> = measurement.zipped();

            chart
                .draw_series(LineSeries::new(
                    zipped_data
                        .iter()
                        .map(|(size, measurement)| ((*size) as i32, *measurement)),
                    &Palette99::pick(measurement_index),
                ))
                .unwrap()
                .label(measurement.name.to_string())
                .legend(move |(x, y)| {
                    PathElement::new(
                        vec![(x, y), (x + 20, y)],
                        Palette99::pick(measurement_index),
                    )
                });
        }

        chart
            .configure_series_labels()
            .background_style(RGBColor(128, 128, 128))
            .draw()
            .expect("Failed to draw chart");
    } else {
        let mut chart = ChartBuilder::on(&root_area)
            .x_label_area_size(x_label_area_size)
            .y_label_area_size(y_label_area_size)
            .right_y_label_area_size(right_y_label_area_size)
            .margin(margin)
            .caption(chart_name, ("sans-serif", title_font_size))
            .build_cartesian_2d(
                min_value_x_axis..max_value_x_axis,
                min_value_y_axis..max_value_y_axis,
            )
            .unwrap();

        chart
            .configure_mesh()
            .x_desc(x_label)
            .y_desc(y_label)
            .draw()
            .unwrap();

        for (measurement_index, measurement) in measurements.iter().enumerate() {
            let zipped_data: Vec<(usize, f32)> = measurement.zipped();

            chart
                .draw_series(LineSeries::new(
                    zipped_data
                        .iter()
                        .map(|(size, measurement)| ((*size) as i32, *measurement)),
                    &Palette99::pick(measurement_index),
                ))
                .unwrap()
                .label(measurement.name.to_string())
                .legend(move |(x, y)| {
                    PathElement::new(
                        vec![(x, y), (x + 20, y)],
                        Palette99::pick(measurement_index),
                    )
                });
        }

        chart
            .configure_series_labels()
            .background_style(RGBColor(128, 128, 128))
            .draw()
            .expect("Failed to draw chart");
    }

    println!("Wrote image to: {}", output_name);
}
