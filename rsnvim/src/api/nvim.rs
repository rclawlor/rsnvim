use rmpv::Value;

use crate::api::*;
use crate::error::Error;
use crate::value_vec;

/// The below implementations are autogenerated using the Neovim API
impl Nvim {
    /// Since: 9
    pub fn get_autocmds(&mut self, opts: Vec<(Value, Value)>) -> Result<Vec<Value>, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_get_autocmds", value_vec!(opts))?;
        Ok(ret
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.to_owned())
            .collect())
    }

    /// Since: 9
    pub fn create_autocmd(
        &mut self,
        event: Value,
        opts: Vec<(Value, Value)>,
    ) -> Result<i64, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_create_autocmd", value_vec!(event, opts))?;
        Ok(ret.as_i64().unwrap())
    }

    /// Since: 9
    pub fn del_autocmd(&mut self, id: i64) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_del_autocmd", value_vec!(id))?;
        Ok(())
    }

    /// Since: 9
    pub fn clear_autocmds(&mut self, opts: Vec<(Value, Value)>) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_clear_autocmds", value_vec!(opts))?;
        Ok(())
    }

    /// Since: 9
    pub fn create_augroup(
        &mut self,
        name: String,
        opts: Vec<(Value, Value)>,
    ) -> Result<i64, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_create_augroup", value_vec!(name, opts))?;
        Ok(ret.as_i64().unwrap())
    }

    /// Since: 9
    pub fn del_augroup_by_id(&mut self, id: i64) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_del_augroup_by_id", value_vec!(id))?;
        Ok(())
    }

    /// Since: 9
    pub fn del_augroup_by_name(&mut self, name: String) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_del_augroup_by_name", value_vec!(name))?;
        Ok(())
    }

    /// Since: 9
    pub fn exec_autocmds(&mut self, event: Value, opts: Vec<(Value, Value)>) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_exec_autocmds", value_vec!(event, opts))?;
        Ok(())
    }

    /// Since: 10
    pub fn parse_cmd(
        &mut self,
        str: String,
        opts: Vec<(Value, Value)>,
    ) -> Result<Vec<(Value, Value)>, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_parse_cmd", value_vec!(str, opts))?;
        Ok(ret
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                let v = x.as_array().unwrap();
                (v[0].to_owned(), v[1].to_owned())
            })
            .collect())
    }

    /// Since: 10
    pub fn cmd(
        &mut self,
        cmd: Vec<(Value, Value)>,
        opts: Vec<(Value, Value)>,
    ) -> Result<String, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_cmd", value_vec!(cmd, opts))?;
        Ok(ret.as_str().unwrap().to_string())
    }

    /// Since: 9
    pub fn create_user_command(
        &mut self,
        name: String,
        command: Value,
        opts: Vec<(Value, Value)>,
    ) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_create_user_command", value_vec!(name, command, opts))?;
        Ok(())
    }

    /// Since: 9
    pub fn del_user_command(&mut self, name: String) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_del_user_command", value_vec!(name))?;
        Ok(())
    }

    /// Since: 4
    pub fn get_commands(
        &mut self,
        opts: Vec<(Value, Value)>,
    ) -> Result<Vec<(Value, Value)>, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_get_commands", value_vec!(opts))?;
        Ok(ret
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                let v = x.as_array().unwrap();
                (v[0].to_owned(), v[1].to_owned())
            })
            .collect())
    }

    /// Since: 5
    pub fn create_namespace(&mut self, name: String) -> Result<i64, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_create_namespace", value_vec!(name))?;
        Ok(ret.as_i64().unwrap())
    }

    /// Since: 5
    pub fn get_namespaces(&mut self) -> Result<Vec<(Value, Value)>, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_get_namespaces", Vec::new())?;
        Ok(ret
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                let v = x.as_array().unwrap();
                (v[0].to_owned(), v[1].to_owned())
            })
            .collect())
    }

    /// Since: 7
    pub fn set_decoration_provider(
        &mut self,
        ns_id: i64,
        opts: Vec<(Value, Value)>,
    ) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_set_decoration_provider", value_vec!(ns_id, opts))?;
        Ok(())
    }

    /// Since: 9
    pub fn get_option_value(
        &mut self,
        name: String,
        opts: Vec<(Value, Value)>,
    ) -> Result<Value, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_get_option_value", value_vec!(name, opts))?;
        Ok(ret.to_owned())
    }

    /// Since: 9
    pub fn set_option_value(
        &mut self,
        name: String,
        value: Value,
        opts: Vec<(Value, Value)>,
    ) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_set_option_value", value_vec!(name, value, opts))?;
        Ok(())
    }

    /// Since: 7
    pub fn get_all_options_info(&mut self) -> Result<Vec<(Value, Value)>, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_get_all_options_info", Vec::new())?;
        Ok(ret
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                let v = x.as_array().unwrap();
                (v[0].to_owned(), v[1].to_owned())
            })
            .collect())
    }

    /// Since: 11
    pub fn get_option_info2(
        &mut self,
        name: String,
        opts: Vec<(Value, Value)>,
    ) -> Result<Vec<(Value, Value)>, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_get_option_info2", value_vec!(name, opts))?;
        Ok(ret
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                let v = x.as_array().unwrap();
                (v[0].to_owned(), v[1].to_owned())
            })
            .collect())
    }

    /// Since: 1
    pub fn ui_attach(
        &mut self,
        width: i64,
        height: i64,
        options: Vec<(Value, Value)>,
    ) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_ui_attach", value_vec!(width, height, options))?;
        Ok(())
    }

    /// Since: 11
    pub fn ui_set_focus(&mut self, gained: bool) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_ui_set_focus", value_vec!(gained))?;
        Ok(())
    }

    /// Since: 1
    pub fn ui_detach(&mut self) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_ui_detach", Vec::new())?;
        Ok(())
    }

    /// Since: 1
    pub fn ui_try_resize(&mut self, width: i64, height: i64) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_ui_try_resize", value_vec!(width, height))?;
        Ok(())
    }

    /// Since: 1
    pub fn ui_set_option(&mut self, name: String, value: Value) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_ui_set_option", value_vec!(name, value))?;
        Ok(())
    }

    /// Since: 6
    pub fn ui_try_resize_grid(&mut self, grid: i64, width: i64, height: i64) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_ui_try_resize_grid", value_vec!(grid, width, height))?;
        Ok(())
    }

    /// Since: 6
    pub fn ui_pum_set_height(&mut self, height: i64) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_ui_pum_set_height", value_vec!(height))?;
        Ok(())
    }

    /// Since: 7
    pub fn ui_pum_set_bounds(
        &mut self,
        width: f64,
        height: f64,
        row: f64,
        col: f64,
    ) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call(
            "nvim_ui_pum_set_bounds",
            value_vec!(width, height, row, col),
        )?;
        Ok(())
    }

    /// Since: 12
    pub fn ui_term_event(&mut self, event: String, value: Value) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_ui_term_event", value_vec!(event, value))?;
        Ok(())
    }

    /// Since: 7
    pub fn get_hl_id_by_name(&mut self, name: String) -> Result<i64, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_get_hl_id_by_name", value_vec!(name))?;
        Ok(ret.as_i64().unwrap())
    }

    /// Since: 11
    pub fn get_hl(
        &mut self,
        ns_id: i64,
        opts: Vec<(Value, Value)>,
    ) -> Result<Vec<(Value, Value)>, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_get_hl", value_vec!(ns_id, opts))?;
        Ok(ret
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                let v = x.as_array().unwrap();
                (v[0].to_owned(), v[1].to_owned())
            })
            .collect())
    }

    /// Since: 7
    pub fn set_hl(
        &mut self,
        ns_id: i64,
        name: String,
        val: Vec<(Value, Value)>,
    ) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_set_hl", value_vec!(ns_id, name, val))?;
        Ok(())
    }

    /// Since: 12
    pub fn get_hl_ns(&mut self, opts: Vec<(Value, Value)>) -> Result<i64, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_get_hl_ns", value_vec!(opts))?;
        Ok(ret.as_i64().unwrap())
    }

    /// Since: 10
    pub fn set_hl_ns(&mut self, ns_id: i64) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_set_hl_ns", value_vec!(ns_id))?;
        Ok(())
    }

    /// Since: 10
    pub fn set_hl_ns_fast(&mut self, ns_id: i64) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_set_hl_ns_fast", value_vec!(ns_id))?;
        Ok(())
    }

    /// Since: 1
    pub fn feedkeys(&mut self, keys: String, mode: String, escape_ks: bool) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_feedkeys", value_vec!(keys, mode, escape_ks))?;
        Ok(())
    }

    /// Since: 1
    pub fn input(&mut self, keys: String) -> Result<i64, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_input", value_vec!(keys))?;
        Ok(ret.as_i64().unwrap())
    }

    /// Since: 6
    pub fn input_mouse(
        &mut self,
        button: String,
        action: String,
        modifier: String,
        grid: i64,
        row: i64,
        col: i64,
    ) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call(
            "nvim_input_mouse",
            value_vec!(button, action, modifier, grid, row, col),
        )?;
        Ok(())
    }

    /// Since: 1
    pub fn replace_termcodes(
        &mut self,
        str: String,
        from_part: bool,
        do_lt: bool,
        special: bool,
    ) -> Result<String, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call(
            "nvim_replace_termcodes",
            value_vec!(str, from_part, do_lt, special),
        )?;
        Ok(ret.as_str().unwrap().to_string())
    }

    /// Since: 7
    pub fn exec_lua(&mut self, code: String, args: Vec<Value>) -> Result<Value, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_exec_lua", value_vec!(code, args))?;
        Ok(ret.to_owned())
    }

    /// Since: 7
    pub fn notify(
        &mut self,
        msg: String,
        log_level: i64,
        opts: Vec<(Value, Value)>,
    ) -> Result<Value, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_notify", value_vec!(msg, log_level, opts))?;
        Ok(ret.to_owned())
    }

    /// Since: 1
    pub fn strwidth(&mut self, text: String) -> Result<i64, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_strwidth", value_vec!(text))?;
        Ok(ret.as_i64().unwrap())
    }

    /// Since: 1
    pub fn list_runtime_paths(&mut self) -> Result<Vec<String>, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_list_runtime_paths", Vec::new())?;
        Ok(ret
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap().to_string())
            .collect())
    }

    /// Since: 7
    pub fn get_runtime_file(&mut self, name: String, all: bool) -> Result<Vec<String>, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_get_runtime_file", value_vec!(name, all))?;
        Ok(ret
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap().to_string())
            .collect())
    }

    /// Since: 1
    pub fn set_current_dir(&mut self, dir: String) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_set_current_dir", value_vec!(dir))?;
        Ok(())
    }

    /// Since: 1
    pub fn get_current_line(&mut self) -> Result<String, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_get_current_line", Vec::new())?;
        Ok(ret.as_str().unwrap().to_string())
    }

    /// Since: 1
    pub fn set_current_line(&mut self, line: String) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_set_current_line", value_vec!(line))?;
        Ok(())
    }

    /// Since: 1
    pub fn del_current_line(&mut self) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_del_current_line", Vec::new())?;
        Ok(())
    }

    /// Since: 1
    pub fn get_var(&mut self, name: String) -> Result<Value, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_get_var", value_vec!(name))?;
        Ok(ret.to_owned())
    }

    /// Since: 1
    pub fn set_var(&mut self, name: String, value: Value) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_set_var", value_vec!(name, value))?;
        Ok(())
    }

    /// Since: 1
    pub fn del_var(&mut self, name: String) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_del_var", value_vec!(name))?;
        Ok(())
    }

    /// Since: 1
    pub fn get_vvar(&mut self, name: String) -> Result<Value, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_get_vvar", value_vec!(name))?;
        Ok(ret.to_owned())
    }

    /// Since: 6
    pub fn set_vvar(&mut self, name: String, value: Value) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_set_vvar", value_vec!(name, value))?;
        Ok(())
    }

    /// Since: 7
    pub fn echo(
        &mut self,
        chunks: Vec<Value>,
        history: bool,
        opts: Vec<(Value, Value)>,
    ) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_echo", value_vec!(chunks, history, opts))?;
        Ok(())
    }

    /// Since: 1
    pub fn out_write(&mut self, str: String) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_out_write", value_vec!(str))?;
        Ok(())
    }

    /// Since: 1
    pub fn err_write(&mut self, str: String) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_err_write", value_vec!(str))?;
        Ok(())
    }

    /// Since: 1
    pub fn err_writeln(&mut self, str: String) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_err_writeln", value_vec!(str))?;
        Ok(())
    }

    /// Since: 1
    pub fn list_bufs(&mut self) -> Result<Vec<Buffer>, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_list_bufs", Vec::new())?;
        Ok(ret
            .as_array()
            .unwrap()
            .iter()
            .map(|x| Buffer {
                data: x.clone(),
                session: self.session.clone(),
            })
            .collect())
    }

    /// Since: 1
    pub fn get_current_buf(&mut self) -> Result<Buffer, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_get_current_buf", Vec::new())?;
        Ok(Buffer {
            data: ret.clone(),
            session: self.session.clone(),
        })
    }

    /// Since: 1
    pub fn set_current_buf(&mut self, buffer: Buffer) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_set_current_buf", value_vec!(buffer))?;
        Ok(())
    }

    /// Since: 1
    pub fn list_wins(&mut self) -> Result<Vec<Window>, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_list_wins", Vec::new())?;
        Ok(ret
            .as_array()
            .unwrap()
            .iter()
            .map(|x| Window {
                data: x.clone(),
                session: self.session.clone(),
            })
            .collect())
    }

    /// Since: 1
    pub fn get_current_win(&mut self) -> Result<Window, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_get_current_win", Vec::new())?;
        Ok(Window {
            data: ret.clone(),
            session: self.session.clone(),
        })
    }

    /// Since: 1
    pub fn set_current_win(&mut self, window: Window) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_set_current_win", value_vec!(window))?;
        Ok(())
    }

    /// Since: 6
    pub fn create_buf(&mut self, listed: bool, scratch: bool) -> Result<Buffer, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_create_buf", value_vec!(listed, scratch))?;
        Ok(Buffer {
            data: ret.clone(),
            session: self.session.clone(),
        })
    }

    /// Since: 7
    pub fn open_term(&mut self, buffer: Buffer, opts: Vec<(Value, Value)>) -> Result<i64, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_open_term", value_vec!(buffer, opts))?;
        Ok(ret.as_i64().unwrap())
    }

    /// Since: 7
    pub fn chan_send(&mut self, chan: i64, data: String) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_chan_send", value_vec!(chan, data))?;
        Ok(())
    }

    /// Since: 1
    pub fn list_tabpages(&mut self) -> Result<Vec<Tabpage>, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_list_tabpages", Vec::new())?;
        Ok(ret
            .as_array()
            .unwrap()
            .iter()
            .map(|x| Tabpage {
                data: x.clone(),
                session: self.session.clone(),
            })
            .collect())
    }

    /// Since: 1
    pub fn get_current_tabpage(&mut self) -> Result<Tabpage, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_get_current_tabpage", Vec::new())?;
        Ok(Tabpage {
            data: ret.clone(),
            session: self.session.clone(),
        })
    }

    /// Since: 1
    pub fn set_current_tabpage(&mut self, tabpage: Tabpage) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_set_current_tabpage", value_vec!(tabpage))?;
        Ok(())
    }

    /// Since: 6
    pub fn paste(&mut self, data: String, crlf: bool, phase: i64) -> Result<bool, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_paste", value_vec!(data, crlf, phase))?;
        Ok(ret.as_bool().unwrap())
    }

    /// Since: 6
    pub fn put(
        &mut self,
        lines: Vec<String>,
        r#type: String,
        after: bool,
        follow: bool,
    ) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_put", value_vec!(lines, r#type, after, follow))?;
        Ok(())
    }

    /// Since: 1
    pub fn subscribe(&mut self, event: String) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_subscribe", value_vec!(event))?;
        Ok(())
    }

    /// Since: 1
    pub fn unsubscribe(&mut self, event: String) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_unsubscribe", value_vec!(event))?;
        Ok(())
    }

    /// Since: 1
    pub fn get_color_by_name(&mut self, name: String) -> Result<i64, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_get_color_by_name", value_vec!(name))?;
        Ok(ret.as_i64().unwrap())
    }

    /// Since: 1
    pub fn get_color_map(&mut self) -> Result<Vec<(Value, Value)>, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_get_color_map", Vec::new())?;
        Ok(ret
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                let v = x.as_array().unwrap();
                (v[0].to_owned(), v[1].to_owned())
            })
            .collect())
    }

    /// Since: 6
    pub fn get_context(&mut self, opts: Vec<(Value, Value)>) -> Result<Vec<(Value, Value)>, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_get_context", value_vec!(opts))?;
        Ok(ret
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                let v = x.as_array().unwrap();
                (v[0].to_owned(), v[1].to_owned())
            })
            .collect())
    }

    /// Since: 6
    pub fn load_context(&mut self, dict: Vec<(Value, Value)>) -> Result<Value, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_load_context", value_vec!(dict))?;
        Ok(ret.to_owned())
    }

    /// Since: 2
    pub fn get_mode(&mut self) -> Result<Vec<(Value, Value)>, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_get_mode", Vec::new())?;
        Ok(ret
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                let v = x.as_array().unwrap();
                (v[0].to_owned(), v[1].to_owned())
            })
            .collect())
    }

    /// Since: 3
    pub fn get_keymap(&mut self, mode: String) -> Result<Vec<Vec<(Value, Value)>>, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_get_keymap", value_vec!(mode))?;
        Ok(ret
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                x.as_array()
                    .unwrap()
                    .iter()
                    .map(|x| {
                        let v = x.as_array().unwrap();
                        (v[0].to_owned(), v[1].to_owned())
                    })
                    .collect()
            })
            .collect())
    }

    /// Since: 6
    pub fn set_keymap(
        &mut self,
        mode: String,
        lhs: String,
        rhs: String,
        opts: Vec<(Value, Value)>,
    ) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_set_keymap", value_vec!(mode, lhs, rhs, opts))?;
        Ok(())
    }

    /// Since: 6
    pub fn del_keymap(&mut self, mode: String, lhs: String) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_del_keymap", value_vec!(mode, lhs))?;
        Ok(())
    }

    /// Since: 1
    pub fn get_api_info(&mut self) -> Result<Vec<Value>, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_get_api_info", Vec::new())?;
        Ok(ret
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.to_owned())
            .collect())
    }

    /// Since: 4
    pub fn set_client_info(
        &mut self,
        name: String,
        version: Vec<(Value, Value)>,
        r#type: String,
        methods: Vec<(Value, Value)>,
        attributes: Vec<(Value, Value)>,
    ) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call(
            "nvim_set_client_info",
            value_vec!(name, version, r#type, methods, attributes),
        )?;
        Ok(())
    }

    /// Since: 4
    pub fn get_chan_info(&mut self, chan: i64) -> Result<Vec<(Value, Value)>, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_get_chan_info", value_vec!(chan))?;
        Ok(ret
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                let v = x.as_array().unwrap();
                (v[0].to_owned(), v[1].to_owned())
            })
            .collect())
    }

    /// Since: 4
    pub fn list_chans(&mut self) -> Result<Vec<Value>, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_list_chans", Vec::new())?;
        Ok(ret
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.to_owned())
            .collect())
    }

    /// Since: 1
    pub fn call_atomic(&mut self, calls: Vec<Value>) -> Result<Vec<Value>, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_call_atomic", value_vec!(calls))?;
        Ok(ret
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.to_owned())
            .collect())
    }

    /// Since: 4
    pub fn list_uis(&mut self) -> Result<Vec<Value>, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_list_uis", Vec::new())?;
        Ok(ret
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.to_owned())
            .collect())
    }

    /// Since: 4
    pub fn get_proc_children(&mut self, pid: i64) -> Result<Vec<Value>, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_get_proc_children", value_vec!(pid))?;
        Ok(ret
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.to_owned())
            .collect())
    }

    /// Since: 4
    pub fn get_proc(&mut self, pid: i64) -> Result<Value, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_get_proc", value_vec!(pid))?;
        Ok(ret.to_owned())
    }

    /// Since: 6
    pub fn select_popupmenu_item(
        &mut self,
        item: i64,
        insert: bool,
        finish: bool,
        opts: Vec<(Value, Value)>,
    ) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call(
            "nvim_select_popupmenu_item",
            value_vec!(item, insert, finish, opts),
        )?;
        Ok(())
    }

    /// Since: 8
    pub fn del_mark(&mut self, name: String) -> Result<bool, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_del_mark", value_vec!(name))?;
        Ok(ret.as_bool().unwrap())
    }

    /// Since: 8
    pub fn get_mark(
        &mut self,
        name: String,
        opts: Vec<(Value, Value)>,
    ) -> Result<Vec<Value>, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_get_mark", value_vec!(name, opts))?;
        Ok(ret
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.to_owned())
            .collect())
    }

    /// Since: 8
    pub fn eval_statusline(
        &mut self,
        str: String,
        opts: Vec<(Value, Value)>,
    ) -> Result<Vec<(Value, Value)>, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_eval_statusline", value_vec!(str, opts))?;
        Ok(ret
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                let v = x.as_array().unwrap();
                (v[0].to_owned(), v[1].to_owned())
            })
            .collect())
    }

    /// Since: 12
    pub fn complete_set(
        &mut self,
        index: i64,
        opts: Vec<(Value, Value)>,
    ) -> Result<Vec<(Value, Value)>, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_complete_set", value_vec!(index, opts))?;
        Ok(ret
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                let v = x.as_array().unwrap();
                (v[0].to_owned(), v[1].to_owned())
            })
            .collect())
    }

    /// Since: 11
    pub fn exec2(
        &mut self,
        src: String,
        opts: Vec<(Value, Value)>,
    ) -> Result<Vec<(Value, Value)>, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_exec2", value_vec!(src, opts))?;
        Ok(ret
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                let v = x.as_array().unwrap();
                (v[0].to_owned(), v[1].to_owned())
            })
            .collect())
    }

    /// Since: 1
    pub fn command(&mut self, command: String) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        session.call("nvim_command", value_vec!(command))?;
        Ok(())
    }

    /// Since: 1
    pub fn eval(&mut self, expr: String) -> Result<Value, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_eval", value_vec!(expr))?;
        Ok(ret.to_owned())
    }

    /// Since: 1
    pub fn call_function(&mut self, r#fn: String, args: Vec<Value>) -> Result<Value, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_call_function", value_vec!(r#fn, args))?;
        Ok(ret.to_owned())
    }

    /// Since: 4
    pub fn call_dict_function(
        &mut self,
        dict: Value,
        r#fn: String,
        args: Vec<Value>,
    ) -> Result<Value, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_call_dict_function", value_vec!(dict, r#fn, args))?;
        Ok(ret.to_owned())
    }

    /// Since: 4
    pub fn parse_expression(
        &mut self,
        expr: String,
        flags: String,
        highlight: bool,
    ) -> Result<Vec<(Value, Value)>, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_parse_expression", value_vec!(expr, flags, highlight))?;
        Ok(ret
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                let v = x.as_array().unwrap();
                (v[0].to_owned(), v[1].to_owned())
            })
            .collect())
    }

    /// Since: 6
    pub fn open_win(
        &mut self,
        buffer: Buffer,
        enter: bool,
        config: Vec<(Value, Value)>,
    ) -> Result<Window, Error> {
        let mut session = self.session.lock().unwrap();
        let ret = session.call("nvim_open_win", value_vec!(buffer, enter, config))?;
        Ok(Window {
            data: ret.clone(),
            session: self.session.clone(),
        })
    }
}
