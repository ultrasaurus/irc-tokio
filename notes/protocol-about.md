# About the IRC Protocol 

* [RFC 2812](https://tools.ietf.org/html/rfc2812) - IRC Client Protocol
* [RFC 2810](https://tools.ietf.org/html/rfc2810) - IRC Architecture

## Background
 Upon connecting to a server, a client registers using a label which
   is then used by other servers and clients to know where the client is
   located.  Servers are responsible for keeping track of all the labels
   being used.

   Servers and clients send each other messages, which may or may not
   generate a reply.  If the message contains a valid command, as
   described in later sections, the client should expect a reply as
   specified but it is not advised to wait forever for the reply; client
   to server and server to server communication is essentially
   asynchronous by nature.

### IRC message format
   Each IRC message may consist of up to three main parts separated by one
   ASCII space character (0x20): 
   1. prefix (OPTIONAL)
   2. command
   3. command parameters (maximum of fifteen (15)).  The prefix, command, and all 
   
   Messages are terminated by CR-LF (Carriage Return - Line Feed) pair

   * Commands MUST either be a valid IRC command or a three (3) digit
   number represented in ASCII text.
   * Messages SHALL NOT exceed 512 characters in length, counting all characters including the trailing CR-LF.
   * NUL (%x00) character is not allowed within messages.
   
#### Numeric replies

   Most of the messages sent to the server generate a reply of some
   sort.  The most common reply is the numeric reply, used for both
   errors and normal replies.  The numeric reply MUST be sent as one
   message consisting of the sender prefix, the three-digit numeric, and
   the target of the reply.  A numeric reply is not allowed to originate
   from a client. In all other respects, a numeric reply is just like a
   normal message, except that the keyword is made up of 3 numeric
   digits rather than a string of letters.  A list of different replies
   is supplied in section 5 (Replies).
   
### Character codes
No specific character set is specified. The protocol is based on a
   set of codes which are composed of eight (8) bits, making up an
   octet.  Each message may be composed of any number of these octets;
   however, some octet values are used for control codes, which act as
   message delimiters.

   Regardless of being an 8-bit protocol, the delimiters and keywords
   are such that protocol is mostly usable from US-ASCII terminal and a
   telnet connection.

   Because of IRC's Scandinavian origin, the characters {}|^ are
   considered to be the lower case equivalents of the characters []\~,
   respectively. This is a critical issue when determining the
   equivalence of two nicknames or channel names.

## Commands required for our bot

PING (3.7.2)
   The PING command is used to test the presence of an active client or
   server at the other end of the connection.  Servers send a PING
   message at regular intervals if no other activity detected coming
   from a connection.  If a connection fails to respond to a PING
   message within a set amount of time, that connection is closed.  A
   PING message MAY be sent even if the connection is active.

   When a PING message is received, the appropriate PONG message MUST be
   sent as reply to <server1> (server which sent the PING message out)
   as soon as possible.  If the <server2> parameter is specified, it
   represents the target of the ping, and the message gets forwarded
   there.