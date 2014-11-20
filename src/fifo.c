/*
 * CDDL HEADER START
 *
 * The contents of this file are subject to the terms of the
 * Common Development and Distribution License (the "License").
 * You may not use this file except in compliance with the License.
 *
 * You can obtain a copy of the license at usr/src/OPENSOLARIS.LICENSE
 * or http://www.opensolaris.org/os/licensing.
 * See the License for the specific language governing permissions
 * and limitations under the License.
 *
 * When distributing Covered Code, include this CDDL HEADER in each
 * file and include the License file at usr/src/OPENSOLARIS.LICENSE.
 * If applicable, add the following below this CDDL HEADER, with the
 * fields enclosed by brackets "[]" replaced with your own identifying
 * information: Portions Copyright [yyyy] [name of copyright owner]
 *
 * CDDL HEADER END
 */
/*
 * Copyright 2011 Joyent, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
/*
 * FiFo zone client, based on https://github.com/joyent/illumos-joyent/blob/master/usr/src/lib/libsmartsshd/common/sshd-plugin.c
 */

#pragma ident "%Z%%M% %I% %E% SMI"

#include <alloca.h>
#include <door.h>
#include <errno.h>
#include <fcntl.h>
#include <pwd.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#define LOG_OOM(SZ)     fprintf(stderr, "Unable to alloca %d bytes\n", SZ)

static const char *DOOR = "/var/tmp/._fifo";
static const char *REQ_FMT_STR = "%s"; /* name uid fp */
static const int RETURN_SZ = 65536;

static const int MAX_ATTEMPTS = 2;
static const int SLEEP_PERIOD = 1;

int
main(int argc, char *argv[])
{
  int fd = -1;
  int blen = 0;
  char *buf = NULL;
  door_arg_t door_args = {0};
  int success = 1;
  blen = strlen(argv[1]) + 1;

  buf = (char *)alloca(blen);
  if (buf == NULL) {
    LOG_OOM(blen);
    return (1);
  }
  strlcpy(buf, argv[1], blen);

  door_args.data_ptr = buf;
  door_args.data_size = blen;

  door_args.rsize = RETURN_SZ;
  door_args.rbuf = alloca(RETURN_SZ);
  if (door_args.rbuf == NULL) {
    LOG_OOM(RETURN_SZ);
    return (1);
  }
  memset(door_args.rbuf, 0, RETURN_SZ);

  fd = open(DOOR, O_RDWR);
  if (fd < 0)
    perror("fifodoor: open (of door FD) failed");

  if (door_call(fd, &door_args) < 0) {
    perror("fifodoor: door_call failed");
    return (1);
  } else {
    success = (*(door_args.rbuf) == '1');
    if (success) {
      fprintf(stdout, "%s\n", (door_args.rbuf + 1));
    } else {
      fprintf(stderr, "error: %s\n", (door_args.rbuf + 1));
    }
    munmap(door_args.rbuf, door_args.rsize);
    return (success);
  }
}
