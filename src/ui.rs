use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph, Wrap},
    Frame,
};

const BG: Color = Color::Rgb(30, 30, 46);
const SURFACE: Color = Color::Rgb(37, 37, 53);
const ACCENT: Color = Color::Rgb(122, 162, 247);
const TEXT: Color = Color::Rgb(192, 194, 245);
const SUBTEXT: Color = Color::Rgb(147, 150, 196);

pub fn render(frame: &mut Frame, placeholder: &str) {
    let [header_area, arena_area, footer_area] = layout(frame.area());

    render_header(frame, header_area);
    render_typing_arena(frame, arena_area, placeholder);
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

fn render_header(frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(ACCENT))
        .style(Style::new().bg(BG));

    let title = Line::from(vec![
        Span::styled("ttybr", Style::new().fg(ACCENT).bold()),
        Span::styled(" — typing tutor", Style::new().fg(SUBTEXT)),
    ])
    .centered();

    let inner = block.inner(area);
    frame.render_widget(block, area);
    frame.render_widget(
        Paragraph::new(title).style(Style::new().bg(BG)),
        inner,
    );
}

fn render_typing_arena(frame: &mut Frame, area: Rect, text: &str) {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(ACCENT))
        .style(Style::new().bg(BG));

    let inner = block.inner(area);

    let paragraph = Paragraph::new(text)
        .style(Style::new().fg(TEXT).bg(BG))
        .centered()
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

fn render_footer(frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(ACCENT))
        .style(Style::new().bg(BG))
        .title(" heatmap ")
        .title_style(Style::new().fg(SUBTEXT));

    frame.render_widget(block, area);
}
