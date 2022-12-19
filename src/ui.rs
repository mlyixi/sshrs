use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table},
    Frame,
};
use unicode_width::UnicodeWidthStr;
pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(10),
            ]
            .as_ref(),
        )
        .split(f.size());
    let help_msg = vec![
        Span::raw("Press "),
        Span::styled("ESC", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to exit, "),
        Span::styled("others", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to search."),
    ];
    let style = Style::default();
    let mut text = Text::from(Spans::from(help_msg));
    text.patch_style(style);
    let help_prg = Paragraph::new(text);
    f.render_widget(help_prg, chunks[0]);

    let search_prg = Paragraph::new(app.searcher.search_string.as_ref())
        .style(Style::default().add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title("Search"));
    f.render_widget(search_prg, chunks[1]);
    f.set_cursor(
        chunks[1].x + app.searcher.search_string.width() as u16 + 1,
        chunks[1].y + 1,
    );

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default();
    let header_cells = ["Host", "User", "Target", "Port", "Jump"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(0);
    let filtered_items = app.get_filtered_items();
    let rows = filtered_items.iter().map(|item| {
        let height = 1;
        let cells = [
            Cell::from(item.host.to_string()),
            Cell::from(item.user.to_string()),
            Cell::from(item.target.to_string()),
            Cell::from(item.port.to_string()),
            Cell::from(item.jump.to_string()),
        ];
        Row::new(cells).height(height).bottom_margin(0)
    });
    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(selected_style)
        .widths(&[
            Constraint::Length(15),
            Constraint::Length(10),
            Constraint::Percentage(30),
            Constraint::Length(10),
            Constraint::Min(10),
        ]);
    f.render_stateful_widget(t, chunks[2], &mut app.state);

    if app.should_show_popup {
        let block = Block::default()
            .title("Set Jumpers \",\" Seperate")
            .borders(Borders::ALL);
        let area = centered_rect(60, 15, size);
        let jumpers_prg = Paragraph::new(app.completer.display_string.as_ref())
            .style(Style::default().add_modifier(Modifier::BOLD))
            .block(block);
        f.set_cursor(
            area.x + app.completer.display_string.width() as u16 + 1,
            area.y + 1,
        );
        f.render_widget(Clear, area);
        f.render_widget(jumpers_prg, area);
    }
}
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
