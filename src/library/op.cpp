/*
 * niepce - library/op.cpp
 *
 * Copyright (C) 2007 Hubert Figuiere
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */


#include "op.h"


namespace library {

	Op::Op(OpType t)
		: m_type(t),
		  m_id(Op::newId())
	{
		
	}

	Op::~Op()
	{

	}

	Op::id_t Op::newId()
	{
		static id_t _id = 0;
		_id++;
		return _id;
	}

}

