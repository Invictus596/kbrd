use std::time::Instant;

use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

const BG: Color = Color::Rgb(30, 30, 46);
const ACCENT: Color = Color::Rgb(122, 162, 247);
const TEXT: Color = Color::Rgb(192, 194, 245);
const SUBTEXT: Color = Color::Rgb(147, 150, 196);
const DIM_TEXT: Color = Color::Rgb(80, 80, 100);
const ERROR_RED: Color = Color::Rgb(255, 85, 85);
const CORRECT_GREEN: Color = Color::Rgb(80, 200, 120);
const CURSOR_BG: Color = Color::Rgb(122, 162, 247);
const CURSOR_FG: Color = Color::Rgb(30, 30, 46);

pub fn render(frame: &mut Frame, app: &App) {
    if app.show_results {
        let [header_area, results_area, footer_area] = layout(frame.area());
        render_header(frame, header_area, app);
        render_results(frame, results_area, app);
        render_footer(frame, footer_area, app);
    } else {
        let [header_area, arena_area, footer_area] = layout(frame.area());
        render_header(frame, header_area, app);
        render_typing_arena(frame, arena_area, app);
        render_footer(frame, footer_area, app);
    }
}

fn layout(area: Rect) -> [Rect; 3] {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(10),
        Constraint::Length(10),
    ])
    .split(area);

    [chunks[0], chunks[1], chunks[2]]
}

fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(ACCENT))
        .style(Style::new().bg(BG));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let elapsed = app
        .session_end
        .unwrap_or_else(Instant::now)
        .duration_since(app.session_start);
    let secs = elapsed.as_secs_f64();
    let wpm = if secs > 0.0 {
        let words = app.cursor as f64 / 5.0;
        (words / secs * 60.0).round()
    } else {
        0.0
    };

    let title = " kbrd — typing tutor ";
    let stats = format!(" errors: {} | wpm: {:.0} ", app.error_count, wpm,);
    let padding = inner.width.saturating_sub((title.len() + stats.len()) as u16);
    let padding_str = " ".repeat(padding as usize);

    let line = Line::from(vec![
        Span::styled(title, Style::new().fg(ACCENT).bold()),
        Span::raw(padding_str),
        Span::styled(stats, Style::new().fg(SUBTEXT)),
    ]);

    frame.render_widget(Paragraph::new(line).style(Style::new().bg(BG)), inner);
}

fn render_typing_arena(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(ACCENT))
        .style(Style::new().bg(BG));

    let inner = block.inner(area);

    let cursor_visible =
        (Instant::now().duration_since(app.session_start).as_millis() / 500) % 2 == 0;

    let mut spans = Vec::new();
    for (i, ch) in app.text.char_indices() {
        let style = if i == app.cursor {
            if cursor_visible {
                Style::new().bg(CURSOR_BG).fg(CURSOR_FG)
            } else {
                match app.states[i] {
                    Some(true) => Style::new().fg(DIM_TEXT),
                    Some(false) => Style::new().fg(ERROR_RED).underlined(),
                    None => Style::new().fg(TEXT),
                }
            }
        } else {
            match app.states[i] {
                None if i > app.cursor => {
                    let dist = i - app.cursor;
                    let t = (1.0 - (dist as f64 / 7.0).min(1.0)).max(0.15);
                    let v = (80.0 + 140.0 * t) as u8;
                    Style::new().fg(Color::Rgb(v, v, v + 15))
                }
                None => Style::new().fg(TEXT),
                Some(true) => Style::new().fg(DIM_TEXT),
                Some(false) => Style::new().fg(ERROR_RED).underlined(),
            }
        };
        spans.push(Span::styled(ch.to_string(), style));
    }

    let text = Text::from(Line::from(spans));
    let paragraph = Paragraph::new(text)
        .style(Style::new().bg(BG))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

    let vertical_pad = inner.height.saturating_sub(1) / 2;
    let padded_area = Rect {
        x: inner.x,
        y: inner.y + vertical_pad,
        width: inner.width,
        height: 1,
    };

    frame.render_widget(block, area);
    frame.render_widget(paragraph, padded_area);
}

fn render_results(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(ACCENT))
        .style(Style::new().bg(BG))
        .title(" results ")
        .title_style(Style::new().fg(SUBTEXT));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let elapsed = app
        .session_end
        .unwrap_or_else(Instant::now)
        .duration_since(app.session_start);
    let secs = elapsed.as_secs_f64();
    let total_chars = app.text.len();
    let wpm = if secs > 0.0 {
        (total_chars as f64 / 5.0 / secs * 60.0).round()
    } else {
        0.0
    };
    let accuracy = if total_chars > 0 {
        ((total_chars - app.error_count) as f64 / total_chars as f64 * 100.0).round()
    } else {
        100.0
    };

    let mut lines = vec![
        Line::from(vec![
            Span::styled("  WPM:       ", Style::new().fg(SUBTEXT)),
            Span::styled(format!("{:.0}", wpm), Style::new().fg(ACCENT).bold()),
        ]),
        Line::from(vec![
            Span::styled("  Accuracy:  ", Style::new().fg(SUBTEXT)),
            Span::styled(
                format!("{:.0}%", accuracy),
                Style::new().fg(CORRECT_GREEN).bold(),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Errors:    ", Style::new().fg(SUBTEXT)),
            Span::styled(
                format!("{}", app.error_count),
                Style::new().fg(ERROR_RED).bold(),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Time:      ", Style::new().fg(SUBTEXT)),
            Span::styled(format!("{:.1}s", secs), Style::new().fg(TEXT)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  Press Enter for next sentence  |  Esc to quit",
            Style::new().fg(DIM_TEXT).italic(),
        )),
    ];

    let mut keys: Vec<_> = app.key_stats.iter().collect();
    keys.sort_by(|a, b| a.0.cmp(b.0));
    if !keys.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  Per-key breakdown:",
            Style::new().fg(SUBTEXT),
        )));
        for (ch, stat) in keys {
            let avg_ms = stat.avg_time().as_secs_f64() * 1000.0;
            let acc = stat.accuracy() * 100.0;
            lines.push(Line::from(vec![
                Span::styled(format!("   '{}'", ch), Style::new().fg(TEXT).bold()),
                Span::styled(
                    format!("  acc: {:.0}%", acc),
                    Style::new().fg(if acc > 90.0 {
                        CORRECT_GREEN
                    } else if acc > 70.0 {
                        Color::Rgb(255, 200, 80)
                    } else {
                        ERROR_RED
                    }),
                ),
                Span::styled(format!("  avg: {:.0}ms", avg_ms), Style::new().fg(SUBTEXT)),
                Span::styled(format!("  hits: {}", stat.hits), Style::new().fg(SUBTEXT)),
            ]));
        }
    }

    let paragraph = Paragraph::new(Text::from(lines))
        .style(Style::new().bg(BG))
        .alignment(Alignment::Left);
    frame.render_widget(paragraph, inner);
}

fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(ACCENT))
        .style(Style::new().bg(BG))
        .title(" heatmap ")
        .title_style(Style::new().fg(SUBTEXT));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let keyboard = render_keyboard(app);
    let paragraph = Paragraph::new(keyboard)
        .style(Style::new().bg(BG))
        .alignment(Alignment::Center);
    frame.render_widget(paragraph, inner);
}

fn render_keyboard(app: &App) -> Vec<Line<'static>> {
    let rows = vec![
        vec!["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"],
        vec!["A", "S", "D", "F", "G", "H", "J", "K", "L"],
        vec!["Z", "X", "C", "V", "B", "N", "M"],
    ];

    rows.into_iter()
        .enumerate()
        .map(|(row_idx, row)| {
            let indent = match row_idx {
                0 => 0,
                1 => 1,
                _ => 2,
            };

            let mut spans = vec![Span::raw(" ".repeat(indent))];

            for key in row {
                let ch = key.to_ascii_lowercase().chars().next().unwrap();
                let color = heatmap_color(ch, app);
                spans.push(Span::styled(
                    format!("[{}] ", key),
                    Style::new().fg(color).bold(),
                ));
            }

            Line::from(spans)
        })
        .collect()
}

fn heatmap_color(ch: char, app: &App) -> Color {
    let stats = app.key_stats.get(&ch);
    match stats {
        None => SUBTEXT,
        Some(s) => {
            let acc = s.accuracy();
            if acc >= 0.95 {
                CORRECT_GREEN
            } else if acc >= 0.80 {
                Color::Rgb(255, 200, 80)
            } else if acc >= 0.60 {
                Color::Rgb(255, 150, 50)
            } else {
                ERROR_RED
            }
        }
    }
}
