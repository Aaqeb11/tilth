variable "instance_type" {
  type        = string
  description = "The size of the EC2 instance"
  default     = "t3.micro"
}

variable "region" {
  type        = string
  description = "AWS region to deploy to"
}

variable "enable_monitoring" {
  type        = bool
  description = "Enable detailed monitoring"
  default     = false
}
