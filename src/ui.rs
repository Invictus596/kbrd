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
const CURSOR_BG: Color = Color::Rgb(122, 162, 247);
const CURSOR_FG: Color = Color::Rgb(30, 30, 46);

pub fn render(frame: &mut Frame, app: &App) {
    let [header_area, arena_area, footer_area] = layout(frame.area());

    render_header(frame, header_area, app);
    render_typing_arena(frame, arena_area, app);
    render_footer(frame, footer_area);
}

fn layout(area: Rect) -> [Rect; 3] {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(10),
        Constraint::Length(8),
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

    let title = " kbrd — typing tutor ";
    let stats = format!(" errors: {} ", app.error_count);
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

    let mut spans = Vec::new();
    for (i, ch) in app.text.char_indices() {
        let style = if i == app.cursor {
            Style::new().bg(CURSOR_BG).fg(CURSOR_FG)
        } else {
            match app.states[i] {
                None => Style::new().fg(TEXT),
                Some(true) => Style::new().fg(DIM_TEXT),
                Some(false) => Style::new().fg(ERROR_RED).underlined(),
            }
        };
        spans.push(Span::styled(ch.to_string(), style));
    }

    let text = Text::from(Line::from(spans));
    let text_height = text.lines.len() as u16;
    let paragraph = Paragraph::new(text)
        .style(Style::new().bg(BG))
        .wrap(Wrap { trim: false });
    let vertical_pad = inner.height.saturating_sub(text_height) / 2;
    let padded_area = Rect {
        x: inner.x,
        y: inner.y + vertical_pad,
        width: inner.width,
        height: inner.height.saturating_sub(vertical_pad * 2).max(text_height),
    };

    frame.render_widget(block, area);
    frame.render_widget(paragraph, padded_area);
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(ACCENT))
        .style(Style::new().bg(BG))
        .title(" heatmap ")
        .title_style(Style::new().fg(SUBTEXT));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let keyboard = render_keyboard();
    let paragraph = Paragraph::new(keyboard)
        .style(Style::new().bg(BG))
        .alignment(Alignment::Center);
    frame.render_widget(paragraph, inner);
}

fn render_keyboard() -> Vec<Line<'static>> {
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
                let color = heatmap_color(key);
                spans.push(Span::styled(
                    format!("[{}] ", key),
                    Style::new().fg(color).bold(),
                ));
            }

            Line::from(spans)
        })
        .collect()
}

fn heatmap_color(key: &str) -> Color {
    match key {
        "E" | "T" => Color::Rgb(80, 200, 120),
        "X" | "P" => Color::Rgb(255, 85, 85),
        _ => Color::Rgb(147, 150, 196),
    }
}
