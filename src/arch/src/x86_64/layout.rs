// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
//
// Portions Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the THIRD-PARTY file.

/// Magic addresses externally used to lay out x86_64 VMs.

/// Initial stack for the boot CPU.
pub const BOOT_STACK_POINTER: u64 = 0x8ff0;

/// Kernel command line start address.
pub const CMDLINE_START: u64 = 0x20000;
/// Kernel command line start address maximum size.
pub const CMDLINE_MAX_SIZE: usize = 0x10000;

/// Start of the high memory.
pub const HIMEM_START: u64 = 0x0010_0000; //1 MB.

// Typically, on x86 systems 24 IRQs are used (0-23).
/// First usable IRQ ID for virtio device interrupts on x86_64.
pub const IRQ_BASE: u32 = 5;
/// Last usable IRQ ID for virtio device interrupts on x86_64.
pub const IRQ_MAX: u32 = 23;

/// Address for the TSS setup.
pub const KVM_TSS_ADDRESS: u64 = 0xfffb_d000;

/// The 'zero page', a.k.a linux kernel bootparams.
pub const ZERO_PAGE_START: u64 = 0x7000;

// ** 32-bit reserved area (start: 3GiB, length: 1GiB) **
pub const MEM_32BIT_RESERVED_START: u64 = 0xc000_0000;
pub const MEM_32BIT_RESERVED_SIZE: u64 = 1024 << 20;

// Sub range: 32-bit PCI devices (start: 3GiB, length: 640Mib)
pub const MEM_32BIT_DEVICES_START: u64 = MEM_32BIT_RESERVED_START;
pub const MEM_32BIT_DEVICES_SIZE: u64 = 640 << 20;

// PCI MMCONFIG space (start: after the device space, length: 256MiB)
pub const PCI_MMCONFIG_START: u64 = MEM_32BIT_DEVICES_START + MEM_32BIT_DEVICES_SIZE;
pub const PCI_MMCONFIG_SIZE: u64 = 256 << 20;
