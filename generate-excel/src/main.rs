extern crate simple_excel_writer;
use simple_excel_writer as excel;

use excel::*;

fn main() {
    let mut wb = Workbook::create("/home/eflorespalma/Workspace/excel.xlsx");
    let mut sheet = wb.create_sheet("SheetName");

    // set column width
    sheet.add_column(Column { width: 30.0 });
    sheet.add_column(Column { width: 30.0 });
    sheet.add_column(Column { width: 80.0 });
    sheet.add_column(Column { width: 60.0 });

    wb.write_sheet(&mut sheet, |sheet_writer| {
        let sw = sheet_writer;
        sw.append_row(row!["Name", "Title","Success","XML"])?;
        sw.append_row(row!["Rebeca", (), true,"<xml><tag>\"Hola\" & 'Excel desde Rust'</tag></xml>"])?;
        sw.append_blank_rows(2);
        sw.append_row(row!["Edgar", blank!(30), "retired"])
    }).expect("write excel error!");

    let mut sheet = wb.create_sheet("Sheet2");
    wb.write_sheet(&mut sheet, |sheet_writer| {
        let sw = sheet_writer;
        sw.append_row(row!["Name", "Title","Success","Remark"])?;
        sw.append_row(row!["eflorespalma", "Manager", true])
    }).expect("write excel error!");

    wb.close().expect("close excel error!");
}