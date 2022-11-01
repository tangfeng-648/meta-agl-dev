/* SPDX-License-Identifier: BSD-3-Clause */
/*
 * Copyright (C) 2021 OpenSynergy GmbH
 */
#ifndef _LINUX_VIRTIO_VIRTIO_CAN_H
#define _LINUX_VIRTIO_VIRTIO_CAN_H

#include <linux/types.h>
#include <linux/virtio_types.h>
#include <linux/virtio_ids.h>
#include <linux/virtio_config.h>

/* Feature bit numbers */
#define VIRTIO_CAN_F_CAN_CLASSIC        0u
#define VIRTIO_CAN_F_CAN_FD             1u
#define VIRTIO_CAN_F_LATE_TX_ACK        2u
#define VIRTIO_CAN_F_RTR_FRAMES         3u

/* CAN Result Types */
#define VIRTIO_CAN_RESULT_OK            0u
#define VIRTIO_CAN_RESULT_NOT_OK        1u

/* CAN flags to determine type of CAN Id */
#define VIRTIO_CAN_FLAGS_EXTENDED       0x8000u
#define VIRTIO_CAN_FLAGS_FD             0x4000u
#define VIRTIO_CAN_FLAGS_RTR            0x2000u

/* TX queue message types */
struct virtio_can_tx_out {
#define VIRTIO_CAN_TX                   0x0001u
	__le16 msg_type;
	__le16 reserved;
	__le32 flags;
	__le32 can_id;
	__u8 sdu[64u];
};

struct virtio_can_tx_in {
	__u8 result;
};

/* RX queue message types */
struct virtio_can_rx {
#define VIRTIO_CAN_RX                   0x0101u
	__le16 msg_type;
	__le16 reserved;
	__le32 flags;
	__le32 can_id;
	__u8 sdu[64u];
};

/* Control queue message types */
struct virtio_can_control_out {
#define VIRTIO_CAN_SET_CTRL_MODE_START  0x0201u
#define VIRTIO_CAN_SET_CTRL_MODE_STOP   0x0202u
	__le16 msg_type;
};

struct virtio_can_control_in {
	__u8 result;
};

/* Indication queue message types */
struct virtio_can_event_ind {
#define VIRTIO_CAN_BUSOFF_IND           0x0301u
	__le16 msg_type;
};

#endif /* #ifndef _LINUX_VIRTIO_VIRTIO_CAN_H */
