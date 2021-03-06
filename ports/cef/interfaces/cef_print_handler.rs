// Copyright (c) 2014 Marshall A. Greenblatt. All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//    * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//    * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//    * Neither the name of Google Inc. nor the name Chromium Embedded
// Framework nor the names of its contributors may be used to endorse
// or promote products derived from this software without specific prior
// written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// ---------------------------------------------------------------------------
//
// This file was generated by the CEF translator tool and should not be edited
// by hand. See the translator.README.txt file in the tools directory for
// more information.
//

#![allow(non_snake_case, unused_imports)]

use eutil;
use interfaces;
use types;
use wrappers::CefWrap;

use libc;
use std::collections::HashMap;
use std::ptr;

//
// Callback structure for asynchronous continuation of print dialog requests.
//
#[repr(C)]
pub struct _cef_print_dialog_callback_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Continue printing with the specified |settings|.
  //
  pub cont: Option<extern "C" fn(this: *mut cef_print_dialog_callback_t,
      settings: *mut interfaces::cef_print_settings_t) -> ()>,

  //
  // Cancel the printing.
  //
  pub cancel: Option<extern "C" fn(this: *mut cef_print_dialog_callback_t) -> (
      )>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
} 

pub type cef_print_dialog_callback_t = _cef_print_dialog_callback_t;


//
// Callback structure for asynchronous continuation of print dialog requests.
//
pub struct CefPrintDialogCallback {
  c_object: *mut cef_print_dialog_callback_t,
}

impl Clone for CefPrintDialogCallback {
  fn clone(&self) -> CefPrintDialogCallback{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefPrintDialogCallback {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefPrintDialogCallback {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefPrintDialogCallback {
  pub unsafe fn from_c_object(c_object: *mut cef_print_dialog_callback_t) -> CefPrintDialogCallback {
    CefPrintDialogCallback {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_print_dialog_callback_t) -> CefPrintDialogCallback {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefPrintDialogCallback {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_print_dialog_callback_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_print_dialog_callback_t {
    unsafe {
      if !self.c_object.is_null() {
        eutil::add_ref(self.c_object as *mut types::cef_base_t);
      }
      self.c_object
    }
  }

  pub fn is_null_cef_object(&self) -> bool {
    self.c_object.is_null()
  }
  pub fn is_not_null_cef_object(&self) -> bool {
    !self.c_object.is_null()
  }

  //
  // Continue printing with the specified |settings|.
  //
  pub fn cont(&self, settings: interfaces::CefPrintSettings) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).cont.unwrap())(
          self.c_object,
          CefWrap::to_c(settings)))
    }
  }

  //
  // Cancel the printing.
  //
  pub fn cancel(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).cancel.unwrap())(
          self.c_object))
    }
  }
} 

impl CefWrap<*mut cef_print_dialog_callback_t> for CefPrintDialogCallback {
  fn to_c(rust_object: CefPrintDialogCallback) -> *mut cef_print_dialog_callback_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_print_dialog_callback_t) -> CefPrintDialogCallback {
    CefPrintDialogCallback::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_print_dialog_callback_t> for Option<CefPrintDialogCallback> {
  fn to_c(rust_object: Option<CefPrintDialogCallback>) -> *mut cef_print_dialog_callback_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_print_dialog_callback_t) -> Option<CefPrintDialogCallback> {
    if c_object.is_null() {
      None
    } else {
      Some(CefPrintDialogCallback::from_c_object_addref(c_object))
    }
  }
}


//
// Callback structure for asynchronous continuation of print job requests.
//
#[repr(C)]
pub struct _cef_print_job_callback_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Indicate completion of the print job.
  //
  pub cont: Option<extern "C" fn(this: *mut cef_print_job_callback_t) -> ()>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
} 

pub type cef_print_job_callback_t = _cef_print_job_callback_t;


//
// Callback structure for asynchronous continuation of print job requests.
//
pub struct CefPrintJobCallback {
  c_object: *mut cef_print_job_callback_t,
}

impl Clone for CefPrintJobCallback {
  fn clone(&self) -> CefPrintJobCallback{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefPrintJobCallback {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefPrintJobCallback {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefPrintJobCallback {
  pub unsafe fn from_c_object(c_object: *mut cef_print_job_callback_t) -> CefPrintJobCallback {
    CefPrintJobCallback {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_print_job_callback_t) -> CefPrintJobCallback {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefPrintJobCallback {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_print_job_callback_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_print_job_callback_t {
    unsafe {
      if !self.c_object.is_null() {
        eutil::add_ref(self.c_object as *mut types::cef_base_t);
      }
      self.c_object
    }
  }

  pub fn is_null_cef_object(&self) -> bool {
    self.c_object.is_null()
  }
  pub fn is_not_null_cef_object(&self) -> bool {
    !self.c_object.is_null()
  }

  //
  // Indicate completion of the print job.
  //
  pub fn cont(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).cont.unwrap())(
          self.c_object))
    }
  }
} 

impl CefWrap<*mut cef_print_job_callback_t> for CefPrintJobCallback {
  fn to_c(rust_object: CefPrintJobCallback) -> *mut cef_print_job_callback_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_print_job_callback_t) -> CefPrintJobCallback {
    CefPrintJobCallback::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_print_job_callback_t> for Option<CefPrintJobCallback> {
  fn to_c(rust_object: Option<CefPrintJobCallback>) -> *mut cef_print_job_callback_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_print_job_callback_t) -> Option<CefPrintJobCallback> {
    if c_object.is_null() {
      None
    } else {
      Some(CefPrintJobCallback::from_c_object_addref(c_object))
    }
  }
}


//
// Implement this structure to handle printing on Linux. The functions of this
// structure will be called on the browser process UI thread.
//
#[repr(C)]
pub struct _cef_print_handler_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Synchronize |settings| with client state. If |get_defaults| is true (1)
  // then populate |settings| with the default print settings. Do not keep a
  // reference to |settings| outside of this callback.
  //
  pub on_print_settings: Option<extern "C" fn(this: *mut cef_print_handler_t,
      settings: *mut interfaces::cef_print_settings_t,
      get_defaults: libc::c_int) -> ()>,

  //
  // Show the print dialog. Execute |callback| once the dialog is dismissed.
  // Return true (1) if the dialog will be displayed or false (0) to cancel the
  // printing immediately.
  //
  pub on_print_dialog: Option<extern "C" fn(this: *mut cef_print_handler_t,
      has_selection: libc::c_int,
      callback: *mut interfaces::cef_print_dialog_callback_t) -> libc::c_int>,

  //
  // Send the print job to the printer. Execute |callback| once the job is
  // completed. Return true (1) if the job will proceed or false (0) to cancel
  // the job immediately.
  //
  pub on_print_job: Option<extern "C" fn(this: *mut cef_print_handler_t,
      document_name: *const types::cef_string_t,
      pdf_file_path: *const types::cef_string_t,
      callback: *mut interfaces::cef_print_job_callback_t) -> libc::c_int>,

  //
  // Reset client state related to printing.
  //
  pub on_print_reset: Option<extern "C" fn(this: *mut cef_print_handler_t) -> (
      )>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
} 

pub type cef_print_handler_t = _cef_print_handler_t;


//
// Implement this structure to handle printing on Linux. The functions of this
// structure will be called on the browser process UI thread.
//
pub struct CefPrintHandler {
  c_object: *mut cef_print_handler_t,
}

impl Clone for CefPrintHandler {
  fn clone(&self) -> CefPrintHandler{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefPrintHandler {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefPrintHandler {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefPrintHandler {
  pub unsafe fn from_c_object(c_object: *mut cef_print_handler_t) -> CefPrintHandler {
    CefPrintHandler {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_print_handler_t) -> CefPrintHandler {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefPrintHandler {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_print_handler_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_print_handler_t {
    unsafe {
      if !self.c_object.is_null() {
        eutil::add_ref(self.c_object as *mut types::cef_base_t);
      }
      self.c_object
    }
  }

  pub fn is_null_cef_object(&self) -> bool {
    self.c_object.is_null()
  }
  pub fn is_not_null_cef_object(&self) -> bool {
    !self.c_object.is_null()
  }

  //
  // Synchronize |settings| with client state. If |get_defaults| is true (1)
  // then populate |settings| with the default print settings. Do not keep a
  // reference to |settings| outside of this callback.
  //
  pub fn on_print_settings(&self, settings: interfaces::CefPrintSettings,
      get_defaults: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_print_settings.unwrap())(
          self.c_object,
          CefWrap::to_c(settings),
          CefWrap::to_c(get_defaults)))
    }
  }

  //
  // Show the print dialog. Execute |callback| once the dialog is dismissed.
  // Return true (1) if the dialog will be displayed or false (0) to cancel the
  // printing immediately.
  //
  pub fn on_print_dialog(&self, has_selection: libc::c_int,
      callback: interfaces::CefPrintDialogCallback) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_print_dialog.unwrap())(
          self.c_object,
          CefWrap::to_c(has_selection),
          CefWrap::to_c(callback)))
    }
  }

  //
  // Send the print job to the printer. Execute |callback| once the job is
  // completed. Return true (1) if the job will proceed or false (0) to cancel
  // the job immediately.
  //
  pub fn on_print_job(&self, document_name: &[u16], pdf_file_path: &[u16],
      callback: interfaces::CefPrintJobCallback) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_print_job.unwrap())(
          self.c_object,
          CefWrap::to_c(document_name),
          CefWrap::to_c(pdf_file_path),
          CefWrap::to_c(callback)))
    }
  }

  //
  // Reset client state related to printing.
  //
  pub fn on_print_reset(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_print_reset.unwrap())(
          self.c_object))
    }
  }
} 

impl CefWrap<*mut cef_print_handler_t> for CefPrintHandler {
  fn to_c(rust_object: CefPrintHandler) -> *mut cef_print_handler_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_print_handler_t) -> CefPrintHandler {
    CefPrintHandler::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_print_handler_t> for Option<CefPrintHandler> {
  fn to_c(rust_object: Option<CefPrintHandler>) -> *mut cef_print_handler_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_print_handler_t) -> Option<CefPrintHandler> {
    if c_object.is_null() {
      None
    } else {
      Some(CefPrintHandler::from_c_object_addref(c_object))
    }
  }
}

